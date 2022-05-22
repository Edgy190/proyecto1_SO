mod thread_city;


fn main() {
    let mut m = thread_city::create_city_matrix();

    thread_city::find_path(6,4,0,4,m);

}
