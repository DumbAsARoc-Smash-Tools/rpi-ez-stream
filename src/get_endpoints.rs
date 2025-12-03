// //! GET Endpoints for overlay applications to use
// //!
// //! API is as follows:
// //!   - /data_json => returns a JSON version of the data in
// //!     the software currently. Used in overlays to get
// //!     data for the webpage to update with

// use warp::Filter;

// /// Port number for applications to connect to
// pub const PORT: u16 = 34898;

// pub async fn run_webserver() {
//     let data_json_filter = warp::get()
//         .and(warp::path!("data_json"))
//         .map(|| {
//             let appdata = crate::application_data::APPLICATION_STATE.lock().unwrap();
//             let json_text = appdata.get_serialized_json().unwrap();
//             json_text
//         });

//     warp::serve(data_json_filter)
//         .run(([127, 0, 0, 1], PORT))
//         .await;
// }
