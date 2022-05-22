use simple_matrix::Matrix;


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
    
    println!("{:?}", zero);
    return zero;

}

pub fn find_path(x1: i8, y1: i8, x2: i8, y2: i8, matrix: Matrix<i8>){
    let mut start_x = x1;
    let mut start_y = y1;
    let mut end_x = x2;
    let mut end_y = y2;
    let mut path:  Vec::<Vec<i8>> = Vec::new();

    path.push([x1,y1].to_vec());               //se agrega la posicion inicial al path

    let n0: i8 = 0;

    while true {

        //en caso de que esten a la par (en cualquiera de las 4 posiciones Norte,Sur,Este,Oeste)
        if start_x == end_x && start_y == end_y {
            break;
        }
        if start_x == end_x && start_y + 1 == end_y {
            println!("{:?} ", "bloque 1");
            start_y += 1;
            path.push([start_x,start_y].to_vec());
        }
        if start_x == end_x && start_y - 1 == end_y {
            println!("{:?} ", "bloque 1");
            start_y -= 1;
            path.push([start_x,start_y].to_vec());
        }
        if start_x + 1 == end_x && start_y == end_y {
            println!("{:?} ", "bloque 1");
            start_x += 1;
            path.push([start_x,start_y].to_vec());
        }
        if start_x - 1 == end_x && start_y == end_y {
            println!("{:?} ", "bloque 1");
            start_x -= 1;
            path.push([start_x,start_y].to_vec());
        }
        //si x1 < x2 debo intentar bajar en la ciudad
        if start_x < end_x{                 
            println!("{:?} ", "bloque 2");
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
            println!("{:?} ", "bloque 3");
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
                    println!("aqui");
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
            println!("{:?} ", "bloque 4");
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

        println!("{:?} {:?}", start_x, start_y);
        
    }
    println!("{:?}", path)





    //path.push([1,2].to_vec());
    //println!("{:?} {:?} {:?}", start_x, start_y, path);
}

