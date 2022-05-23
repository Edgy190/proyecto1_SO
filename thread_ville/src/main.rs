mod thread_city;


fn main() {
    //replicas de la ciudad
    let city = thread_city::create_city_matrix();
    let city2 = thread_city::create_city_matrix();
    let city3 = thread_city::create_city_matrix();

    //prueba con un carro
    let add_car = thread_city::add_car_to_city(city);
    thread_city::follow_path(add_car, city2,2);

    //prueba con un barco
    //let add_ship = thread_city::add_ship_to_city();
    //thread_city::follow_path(add_ship, city3,4);

    //prueba con una ambulancia
    //let add_car = thread_city::add_car_to_city(city);
    //thread_city::follow_path(add_car, city2,4);


    /*NOTA: Se puede probar con un carro y barco o ambulancia y barco a la vez
            pero no se puede carro y ambulancia a la vez por problemas de 
            apropiamiento de rust en cuanto a variables/funciones
    */

}
