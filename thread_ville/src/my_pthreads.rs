use rand::prelude::*;
use libc::ucontext_t;
use libc::setcontext;
use libc::getcontext;
use libc::makecontext;
use libc::swapcontext;
use libc::sigemptyset;
use libc::malloc;

//Definiciones globales
const number_threads: i32 = 100;
const stacksize: usize = 4096;
const intervalo: i32 = 100;

pub struct MyPThread {
    //Variables ucontext
    pub signal_context: ucontext_t,
    pub threads: (vec![number_threads], ucontext_t),
    pub current_thread: ucontext_t,
    pub exit_context: ucontext_t,

    //Variables [int]
    pub priority: vec![number_threads],
    pub priority_aux: vec![number_threads],
    pub tickets: vec![number_threads],
    pub dead_threads: vec![number_threads],

    //Variables int
    pub curr_context: u32,
    pub init: u32,
    pub active_threads: u32,
    pub active_threads_aux: u32,
    pub total_tickets: u32,
    pub active_scheduler: u32,
}

impl MyPThread {
///////////////////////////////////////////////////////////////////////////////////////////////MyPThread methods
//pub fn my_thread_create (mut self, mut funcion: &dyn Fn() -> (), mut args: &dyn Fn() -> (), mut tickets_sched: u32, mut priority_sched: u32) -> ()
    pub fn my_thread_create(mut self, mut funcion: extern "C" fn() -> (), mut args: extern "C" fn() -> (),
                            mut tickets_sched: u32, mut priority_sched: u32) -> () {
        if self.init == 0 {
            //setup()
        }
        let &stack;
        let context: ucontext_t;
        //Contexto del hilo
        let uc = &self.threads[self.active_threads];
        getcontext(uc); //Inicializa el contexto
        let stack = malloc(stacksize);
        uc = self.threads.uc_stack.ss_sp = stack;
        uc = self.threads.uc_stack.ss_size = stacksize;
        uc = self.threads.uc_stack.ss_flags = 0;
        uc = self.threads.uc_link = &self.exit_context; //Se reinicia antes de volver hacer el contexto
        
        sigemptyset(self.threads.uc_sigmask);//inicializa la senal
        makecontext(uc, funcion, 1, args);//Realiza el contexto

        /*sigemptyset(&uc->uc_sigmask);
        printf("%s", uc); CODIGO EN C FUENTE*/

        //Set para los schedulers
        self.tickets[self.active_threads] = tickets_sched; //Tickets del scheduler
        self.priority[self.active_threads] = priority_sched; //Prioridad del scheduler
        self.total_tickets += tickets_sched;
        self.active_threads += 1;
        self.active_threads_aux += 1;
    }

    pub fn my_thread_end(mut self) -> () {
        self.dead_threads[self.curr_context as usize] = 1;
        self.total_tickets -= self.tickets[self.curr_context as usize];
        self.active_threads_aux -= 1;
        self.interrupt();//Selecciona el scheduler
    }

    pub fn my_thread_yield(mut self) -> () {
        self.interrupt();//Selecciona el scheduler
    }

    pub fn my_thread_join(mut self) -> () {}

    pub fn my_thread_detach(mut self) -> () {}

///////////////////////////////////////////////////////////////////////////////////////////////Schedulers
    pub fn my_thread_chsched(mut self, scheduler: &mut str) -> () { //cambia el scheduler en uso
        if scheduler != "RoundRobin" {
            self.active_scheduler = 0;
        }
        if scheduler != "Sorteo" {
            self.active_scheduler = 1;
        }
        if scheduler != "Real" {
            self.active_scheduler = 2;
        }
    }

    pub fn roundRobin(mut self) -> () {
        if self.active_threads_aux > 0 {
            self.curr_context = (self.curr_context + 1);
            if self.dead_threads[(self.curr_context % self.active_threads) as usize] > 0 {
                while self.dead_threads[(self.curr_context % self.active_threads) as usize] >= 0 {//incrementa el valor
                    //del hilo de contexto con el fin de darle la prioridad
                    self.curr_context += 1;
                }
            }
            self.curr_context = self.curr_context % self.active_threads;
            let current_thread = &self.threads[self.curr_context];
            setcontext(current_thread);//Activa el nuevo hilo segun el contexto
        }
    }

    pub fn sorteo(mut self) -> () {
        let mut random_number: u32 = rand::random(); //valor random
        let mut aux_number: u32 = 0;
        let current_thread;
        if self.active_threads_aux > 0{
            let ganador: u32 = random_number % (self.total_tickets + 1);//Ganador
            aux_number = ganador;
            for mut i in 0..self.active_threads {//Revisa al ganador
                aux_number -= self.tickets[i as usize]; //Resta el numero generado por la cantidad de tickets del hilo
                if aux_number <= 0 {
                    if self.dead_threads[(i % self.active_threads) as usize] > 0 {
                        while self.dead_threads[(i % self.active_threads) as usize] > 0 {
                            i += 1;
                        } //Aumenta la cantidad de hilos muertos/dormidos
                    }
                    self.curr_context = i;
                    current_thread = &self.threads[self.curr_context];
                    break;
                }else{
                    self.tickets[i as usize] += 1;
                    self.total_tickets += 1;
                }
            }
            setcontext(current_thread);//Activa el nuevo hilo bajo un contexto
        }
    }
    
    pub fn real(mut self) -> () {
        let mut aux_number: i32 = -1;
        if self.active_threads_aux > 0 { //verifica si hay hilos activos
            let mut i: u32 = 0;
            for i in 0..self.active_threads {//Busca el hilo de mayor prioridad pendiente a finalizar
                if aux_number < self.priority[i] &&! self.dead_threads[i] &&! self.priority_aux[i] { //Compara AND NOT
                    self.curr_context = i; //Cambia la variable de contexto segun el hilo
                    aux_number = self.priority[i]; //Iguala el numero auxiliar al valor de la prioridad
                }
            }
            if aux_number == -1 {
                for i in 0..self.active_threads {//setea la prioridad a 0
                    self.priority_aux[i] = 0;
                }
                self.real(); //vuelve a llamar a la funcion
            } else {
                self.priority_aux[self.curr_context] = 1;// Hilo que fue ejecutado
                let current_thread = &self.threads[self.curr_context];
                setcontext(current_thread); //Activa el nuevo hilo bajo un contexto
            }
        }
    }

    pub fn interrupt(mut self) -> () {
        getcontext(&self.threads.signal_context);
        self.signal_context.uc_stack.ss_sp = self.threads.signal_stack;
        self.signal_context.uc_stack.ss_size = stacksize;
        self.signal_context.uc_stack.ss_flags = 0;
        sigemptyset(&self.threads.signal_context.uc_sigmask); //inicializa la senal
        /*//Si es de round robin
        if self.active_scheduler == 0 {
            makecontext(&self.threads.signal_context, &self.roundRobin(), 1); //hace un contexto
        }
        //Si es de sorteo
        if self.active_scheduler == 1 {
            makecontext(&self.threads.signal_context, &self.sorteo(), 1); //hace un contexto
        }
        //Si es de tiempo real
        if self.active_scheduler == 2 {
            makecontext(&self.threads.signal_context, &self.sched_real(), 1); //hace un contexto
        }*/
        //Cambia el contexto
        swapcontext(&self.threads.current_thread, &self.threads.signal_context);
    }
}
