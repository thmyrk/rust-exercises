fn toggle_customer_presence() {}

pub mod hosting;

pub mod serving {
    pub fn take_order() {
        call_waiter();
    }

    pub fn serve_order() {
        call_waiter();
    }

    pub fn take_payment() {
        call_waiter();
        crate::front_of_house::toggle_customer_presence();
    }

    fn call_waiter() {}
}
