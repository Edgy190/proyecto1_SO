use simple_matrix::Matrix;
use rand::Rng;


/*
Function: create_city_matrix
Input: -
Output: Matrix<i8>
Description: Esta funcion se encarga de crear la matriz que representa la ciudad. Los 0's indican
             camino posible para carros y ambulancias. Los 1s indican que hay edificios que no se
             pueden atravesar por vehiculos.
*/
pub fn create_city_matrix() -> Matrix<i8> {
    let mut zero: Matrix<i8> = Matrix::new(7, 6);
    
    // se definen donde van a estar los edificios de la ciudad
    //comercios
    zero.set(0, 1, 1);
    zero.set(0, 4, 1);
    zero.set(1, 1, 1);
    zero.set(2, 3, 1);
    zero.set(4, 1, 1);
    zero.set(5, 3, 1);
    zero.set(6, 1, 1);
    zero.set(6, 5, 1);

    //planta nuclear
    zero.set(0, 5, 1);

    //atracadero
    zero.set(3, 1, 1);

    //zona de rio
    zero.set(3, 3, 1);
    
    //println!("{:?}", zero);
    return zero;

}

/*
Function: find_path
Input: x1, y1, x2, y2, Matrix<i8>
Output: Vec<Vec<i8>>
Description: Esta funcion se encarga encontrar el camino a seguir por un carro o ambulancia dado
             un punto de inicio y un punto de llegada (coordenadas). 
             La idea es primero igualar la fila en la que se encuentra el destino y luego moverse
             por la ciudad a la izquierda o derecha segun la posicion de las coordenadas y.
             Retorna un vector con vectores que contienen las coordenadas a seguir.
*/
pub fn find_path(x1: i8, y1: i8, x2: i8, y2: i8, matrix: Matrix<i8>) -> Vec<Vec<i8>>{
    let mut start_x = x1;
    let mut start_y = y1;
    let end_x = x2;
    let end_y = y2;
    let mut path: Vec::<Vec<i8>> = Vec::new();

    path.push([x1,y1].to_vec());               //se agrega la posicion inicial al path

    let n0: i8 = 0;

    loop {

        //en caso de que esten a la par (en cualquiera de las 4 posiciones Norte,Sur,Este,Oeste)
        if start_x == end_x && start_y == end_y {
            break;
        }
        if start_x == end_x && start_y + 1 == end_y {
            start_y += 1;
            path.push([start_x,start_y].to_vec());
        }
        if start_x == end_x && start_y - 1 == end_y {
            start_y -= 1;
            path.push([start_x,start_y].to_vec());
        }
        if start_x + 1 == end_x && start_y == end_y {
            start_x += 1;
            path.push([start_x,start_y].to_vec());
        }
        if start_x - 1 == end_x && start_y == end_y {
            start_x -= 1;
            path.push([start_x,start_y].to_vec());
        }
        //si x1 < x2 debo intentar bajar en la ciudad
        if start_x < end_x{                 

            if matrix.get((start_x+1).try_into().unwrap(), start_y.try_into().unwrap()).unwrap() == &n0 {          //si abajo es seguro go
                start_x += 1;
                path.push([start_x,start_y].to_vec());
            } else {                                         //si no, pruebo derecha o izquierda
                
                if start_y == 0 {                   //las unicas opciones son arriba o derecha
                    if matrix.get(start_x.try_into().unwrap(), (start_y+1).try_into().unwrap()).unwrap() == &n0 {
                        start_y += 1;
                        path.push([start_x,start_y].to_vec());
                    } else {
                        start_x -= 1;
                        path.push([start_x,start_y].to_vec());
                    }
                }
                if start_y == 5 {            //las unicas opciones son izquierda o arriba
                    if matrix.get(start_x.try_into().unwrap(), (start_y-1).try_into().unwrap()).unwrap() == &n0 {
                        start_y -= 1;
                        path.push([start_x,start_y].to_vec());
                    } else {
                        start_x -= 1;
                        path.push([start_x,start_y].to_vec());
                    }
                }
                else{               //no estoy en los bordes, me puedo mover derecha o izquierda
                    if start_y < end_y {
                        //prioridad derecha
                        if matrix.get(start_x.try_into().unwrap(), (start_y+1).try_into().unwrap()).unwrap() == &n0 {
                            start_y += 1;
                            path.push([start_x,start_y].to_vec());
                        } else {
                            start_y -= 1;
                            path.push([start_x,start_y].to_vec());
                        }
                    } else {
                        //prioridad izquierda
                        if matrix.get(start_x.try_into().unwrap(), (start_y-1).try_into().unwrap()).unwrap() == &n0 {
                            start_y -= 1;
                            path.push([start_x,start_y].to_vec());
                        } else {
                            start_y -= 1;
                            path.push([start_x,start_y].to_vec());
                        }
                    }
                }
                
                
            }
        }
        //debo intentar subir en la ciudad
        if start_x > end_x {                 

            if matrix.get((start_x-1).try_into().unwrap(), start_y.try_into().unwrap()).unwrap() == &n0 {          //si arriba es seguro go
                start_x -= 1;
                path.push([start_x,start_y].to_vec());
            } else {                                         

                if start_y == 0 {                   //las unicas opciones son abajo o derecha
                    if matrix.get((start_x+1).try_into().unwrap(), start_y.try_into().unwrap()).unwrap() == &n0 {
                        start_x += 1;
                        path.push([start_x,start_y].to_vec());
                    } else {
                        start_y += 1;
                        path.push([start_x,start_y].to_vec());
                    }
                }
                if start_y == 5 {            //las unicas opciones son izquierda o abajo

                    if matrix.get(start_x.try_into().unwrap(), (start_y-1).try_into().unwrap()).unwrap() == &n0 {
                        start_y -= 1;
                        path.push([start_x,start_y].to_vec());
                    } else {
                        start_x += 1;
                        path.push([start_x,start_y].to_vec());
                    }
                }
                else{               //no estoy en los bordes, me puedo mover derecha o izquierda
                    if start_y < end_y {
                        //prioridad derecha
                        if matrix.get(start_x.try_into().unwrap(), (start_y+1).try_into().unwrap()).unwrap() == &n0 {
                            start_y += 1;
                            path.push([start_x,start_y].to_vec());
                        } else {
                            start_y -= 1;
                            path.push([start_x,start_y].to_vec());
                        }
                    } else {
                        //prioridad izquierda
                        if matrix.get(start_x.try_into().unwrap(), (start_y-1).try_into().unwrap()).unwrap() == &n0 {
                            start_y -= 1;
                            path.push([start_x,start_y].to_vec());
                        } else {
                            start_y -= 1;
                            path.push([start_x,start_y].to_vec());
                        }
                    }
                }
                
                
            }
        }
        //misma fila pero y1 < y2
        if start_x == end_x && start_y < end_y {

            if matrix.get(start_x.try_into().unwrap(), (start_y+1).try_into().unwrap()).unwrap() == &n0 {          //si derecha es seguro go
                start_y += 1;
                path.push([start_x,start_y].to_vec());
            } else { 

                if start_x == 0 && start_y == 0{
                    path.push([start_x+1,start_y].to_vec());
                    path.push([start_x+2,start_y].to_vec());
                    path.push([start_x+2,start_y+1].to_vec());
                    path.push([start_x+2,start_y+2].to_vec());
                    start_x += 2;
                    start_y += 2;
                    
                }
                if start_x == 6 && start_y == 0{
                    path.push([start_x-1,start_y].to_vec());
                    path.push([start_x-1,start_y+1].to_vec());
                    path.push([start_x-1,start_y+2].to_vec());
                    path.push([start_x,start_y+2].to_vec());
                    start_y += 2;
                    
                }
            }
        }
        //misma fila pero y1 > y2
        if start_x == end_x && start_y > end_y { 
            start_y -= 1;
            path.push([start_x,start_y].to_vec());
        }

        //println!("{:?} {:?}", start_x, start_y);
        
    }
    //println!("{:?}", path);

    return path;





    //path.push([1,2].to_vec());
    //println!("{:?} {:?} {:?}", start_x, start_y, path);
}


/*
Function: random_coordinates
Input: option
Output: Vec<Vec<i8>> 
Description: Esta funcion se encarga de seleccionar coordenadas aleatorias que funcionan
             como punto de inicio y fin del recorrido de un vehiculo. Recibe un 0 para 
             las coordenadas x y cualquier numero para las coordenadas y. Estas ultimas
             son especificas ya que los destinos son los comercios/
             Retorna un vector con otro dentro que contiene las coordenadas aleatorias.
*/
pub fn random_coordinates(option: i8) -> Vec<i8>{
    //si la opcion es 0 quiere coordenadas aleatorias para inicio, en otro caso quiere para fin
    let n0: i8 = 0;
    let n3: i8 = 3;
    let mut coordinates: Vec<i8> = Vec::new();
    let posible_ycoordinates = vec![[0,1],[1,1],[2,3],[0,4],[4,1],[6,1],[5,3],[6,5]];

    if option == n0 {           //coordenadas para inicio
        let mut x: i8 = 3;
        while x == n3 {
            x = rand::thread_rng().gen_range(0..7) as i8;
        }
        let y = rand::thread_rng().gen_range(0..6) as i8;

        coordinates.push(x);
        coordinates.push(y);
    }
    else {                      //coordenadas para final
        let r = rand::thread_rng().gen_range(0..8) as i8;
        coordinates = posible_ycoordinates[r as usize].to_vec();
    }

    //println!("{:?}",coordinates);

    return coordinates;
} 

/*
Function: add_ship_to_city
Input: -
Output: Vec<Vec<i8>> 
Description: Esta funcion se encarga de devolver la unica ruta posible que pueden tener los 
             los barcos dentro de la ciudad partiendo de que navegan a traves del puente 1.
             Ademas, imprime el inicio, fin y ruta a seguir en consola.
*/
pub fn add_ship_to_city() -> Vec<Vec<i8>> { 
    let mut ship_path: Vec::<Vec<i8>> = Vec::new();
    ship_path.push([3,0].to_vec());
    ship_path.push([3,1].to_vec());
    ship_path.push([3,2].to_vec());
    ship_path.push([3,3].to_vec());
    ship_path.push([3,4].to_vec());
    ship_path.push([3,5].to_vec());

    println!();
    println!("Inicio del camino: {:?}",ship_path.get(0));
    println!("Fin del camino: {:?}",ship_path.get(5));
    println!("Camino seguido: {:?}", ship_path);
    println!();
    return ship_path;
}

/*
Function: add_car_to_city
Input: city
Output: Vec<Vec<i8>> 
Description: Esta funcion se encarga de devolver una ruta posible para un vehiculo entre unas coordenadas de
             inicio y otras de final. Ademas, imprime el inicio, fin y ruta a seguir en consola.
*/
pub fn add_car_to_city(city: Matrix<i8>) -> Vec<Vec<i8>> {         //funciona tanto para carro como ambulancia!!!
    
    let start = random_coordinates(0);
    let end = random_coordinates(1);

    let car_path = find_path(start[0], start[1], end[0], end[1], city);

    println!();
    println!("Inicio del camino: {:?}",start);
    println!("Fin del camino: {:?}",end);
    println!("Camino seguido: {:?}", car_path);
    println!();
    return car_path;
}


/*
Function: follow_path
Input: path, city, option
Output: Vec<Vec<i8>> 
Description: Esta funcion se encarga de manejar el posible recorrido por un vehiculo o barco dentro de la ciudad.
             Toma en cuenta distintas acciones dependiendo de la zona en la que se encuentre actualmente, por ejemplo
             los semaforos, el ceda, el puente elevadizo, etc.
             Muestra en consola los cambios que se realizan en la ciudad con numeros: un 2 para movimiento de carros,
             un 3 para ambulancias y un 4 para barcos.
*/
pub fn follow_path(path: Vec<Vec<i8>>, mut city: Matrix<i8>, option: i8) {  //option -> 2 para carro, 3 ambulancia y 4 barco
    //let mut bridge_flag = 0;

    for coordinate in path {

        if coordinate[0] == 4 && coordinate[1] == 0 || coordinate[0] == 2 && coordinate[1] == 0 {
            city.set(coordinate[0] as usize, coordinate[1] as usize,option);
            if option == 2 {
                println!("Esperando 5segundos a que cambie el semaforo!");
            }
            else{
                println!("Ambulancia con prioridad en camino!");
            }
        }
        if coordinate[0] == 4 && coordinate[1] == 2 {
            city.set(coordinate[0] as usize, coordinate[1] as usize,option);
            if option == 2 {
                println!("Esperando 3segundos al ceda el paso!");
            }
            else{
                
                println!("Ambulancia con prioridad en camino!");
            }
            
        }
        if coordinate[0] == 2 && coordinate[1] == 2 {
            city.set(coordinate[0] as usize, coordinate[1] as usize,option);
            println!("Ambulancia con prioridad en camino!");
        }
        if coordinate[0] == 3 && coordinate[1] == 3 {
            city.set(coordinate[0] as usize, coordinate[1] as usize,option);
            //bridge_flag += 1;
            //se crea una duracion de varios segundos para que no se pueda pasar por el puente!!
            println!("Deshabilitando puente! Barco en camino.");
            println!("Puente activo en 3");
            println!("Puente activo en 2");
            println!("Puente activo en 1");
            //se resetea la bandera del barco pasando
            //bridge_flag = 0;
        }
        if coordinate[0] == 3 && coordinate[1] == 1 {
            city.set(coordinate[0] as usize, coordinate[1] as usize,option);
            println!("El barco ha llegado al atracadero!")
        }
        if coordinate[0] == 4 && coordinate[1] == 4 || coordinate[0] == 4 && coordinate[1] == 5 || 
        coordinate[0] == 2 && coordinate[1] == 4 || coordinate[0] == 2 && coordinate[1] == 5 { 
            city.set(coordinate[0] as usize, coordinate[1] as usize,option);
            if option == 4 {
                println!("Ambulancia con prioridad en camino!");
            }
        }
        else {
            city.set(coordinate[0] as usize, coordinate[1] as usize,option);
        }

    }

    //println!("{:?}", path);

    //imprimir ciudad
    println!();
    println!("{:?} {:?} {:?} {:?} {:?} {:?}", city.get(0,0).unwrap(),city.get(0,1).unwrap(),city.get(0,2).unwrap(),
    city.get(0,3).unwrap(),city.get(0,4).unwrap(),city.get(0,5).unwrap());

    println!("{:?} {:?} {:?} {:?} {:?} {:?}", city.get(1,0).unwrap(),city.get(1,1).unwrap(),city.get(1,2).unwrap(),
    city.get(1,3).unwrap(),city.get(1,4).unwrap(),city.get(1,5).unwrap());

    println!("{:?} {:?} {:?} {:?} {:?} {:?}", city.get(2,0).unwrap(),city.get(2,1).unwrap(),city.get(2,2).unwrap(),
    city.get(2,3).unwrap(),city.get(2,4).unwrap(),city.get(2,5).unwrap());

    println!("{:?}", " ");

    println!("{:?} {:?} {:?} {:?} {:?} {:?}", city.get(3,0).unwrap(),city.get(3,1).unwrap(),city.get(3,2).unwrap(),
    city.get(3,3).unwrap(),city.get(3,4).unwrap(),city.get(3,5).unwrap());

    println!("{:?}", " ");

    println!("{:?} {:?} {:?} {:?} {:?} {:?}", city.get(4,0).unwrap(),city.get(4,1).unwrap(),city.get(4,2).unwrap(),
    city.get(4,3).unwrap(),city.get(4,4).unwrap(),city.get(4,5).unwrap());

    println!("{:?} {:?} {:?} {:?} {:?} {:?}", city.get(5,0).unwrap(),city.get(5,1).unwrap(),city.get(5,2).unwrap(),
    city.get(5,3).unwrap(),city.get(5,4).unwrap(),city.get(5,5).unwrap());

    println!("{:?} {:?} {:?} {:?} {:?} {:?}", city.get(6,0).unwrap(),city.get(6,1).unwrap(),city.get(6,2).unwrap(),
    city.get(6,3).unwrap(),city.get(6,4).unwrap(),city.get(6,5).unwrap());
}
