use final_record_gui::*;

fn main() -> iced::Result {
    let connection = &mut establish_connection();
    let _ = delete_all();
    let c_date = date_print();
    create_review(connection, "Tour de, France", &5.5, &c_date);
    run_interface()
}
