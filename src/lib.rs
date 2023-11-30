#[no_mangle]
extern "system" fn DllMain(_: *const u8, _: u32, _: *const u8) -> u32 { 1 }

#[derive(Default)]
#[repr(C)]
pub struct Component {
    count: f64,
}

impl Component {
    pub fn eval(&mut self, params: &mut [Parameter]) {
        self.count += 1.0;

        // Parameters are positional based on the index of the pin
        // of the Ø symbol in QSpice
        // Input pins can be read, and output pins can be written
        // to update the QSpice simulation.
        params[7].d = self.count;
    }
}

#[repr(C)]
pub union Parameter {
    b: bool,
    c: u8,
    f: f32,
    d: f64,
}

// NOTE: The name of the evaluation function has to be the same as the .dll
#[no_mangle]
pub extern "C" fn qspice_rs(state: *mut *mut Component, t: f64, param: *mut Parameter) {

    unsafe {
        if (*state).is_null() {
            // Create a boxed Component and pass a raw pointer back to QSpice.
            // This pointer will be provided back to the evaluation function
            // on each time step.
            let inst: Box<Component> = Box::new(Component::default());
            *state = Box::into_raw(inst);
        }

        // Get a slice from the raw pointer to array of Parameters
        // There is no way to know the number of params, so the
        // length argument must be set to the number of pins defined
        // on the Ø symbol in QSpice
        let data = std::slice::from_raw_parts_mut(param, 8);

        // Call the eval func of the Component struct for this time step
        (**state).eval(data);
    }

}

#[no_mangle]
pub extern "C" fn Destroy(pstate: *mut Component) {
    // Convert back to a Box from the raw pointer to dellocate the Component
    unsafe {
        let _state = Box::from_raw(pstate);
    }
}