use rand::prelude::*;
use libc::ucontext_t;

pub struct MyPThread {
    //ucontext
    pub priority: Vec<u32>,
    pub priority_aux: Vec<u32>,
    pub tickets: Vec<u32>,
    pub dead_threads: Vec<u32>,
    //Variables int
    pub curr_context: u32,
    pub init: u32,
    pub active_threads: u32,
    pub active_threads_aux: u32,
    pub total_tickets: u32,
    pub active_scheduler: u32
}

impl MyPThread {
///////////////////////////////////////////////////////////////////////////////////////////////MyPThread methods
    pub fn my_thread_create(mut self, mut funcion: &dyn Fn() -> (), mut args: &dyn Fn() -> (),
                            mut tickets_sched: u32, mut priority_sched: u32) -> () {
        if self.init == 0 {
            //setup()
        }/*
            void *stack;
            //Crea el contexto del hilo
            ucontext_t *uc = &threads[active_threads];
            getcontext(uc); //Inicializa el context
            stack = malloc(STACKSIZE);
            uc->uc_stack.ss_sp = stack;
            uc->uc_stack.ss_size = STACKSIZE;
            uc->uc_stack.ss_flags = 0;
            uc->uc_link = &exitContext; //Se reinicia antes de volver hacer el context
            sigemptyset(&uc->uc_sigmask);
            printf("%s", uc);
            makecontext(uc, function, 1, args);
            //Setea los datos para los schedulers
            tickets[active_threads] = tickets_sched; //Tickets del scheduler
            priority[active_threads] = priority_sched; //Prioridad del scheduler
            total_tickets+=tickets_sched;
            active_threads++;
            active_threads_aux++;
            printf("Se creo un hilo de manera correcta");*/
    }

    pub fn my_thread_end(mut self) -> () {
        self.dead_threads[self.curr_context as usize] = 1;
        self.total_tickets -= self.tickets[self.curr_context as usize];
        self.active_threads_aux -= 1;
        self.timer_interrupt();//Seleccionar el scheduler
    }

    pub fn my_thread_yield(mut self) -> () {
        self.timer_interrupt();//Seleccionar el scheduler
    }

///////////////////////////////////////////////////////////////////////////////////////////////Schedulers
    pub fn my_thread_chsched(mut self, scheduler: &mut str) -> () {
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
                while self.dead_threads[(self.curr_context % self.active_threads) as usize] >= 0 {
                    self.curr_context += 1;
                }
            }
            self.curr_context = self.curr_context % self.active_threads;
            //current_thread = &self.threads[self.curr_context];
            //setcontext(current_thread);//Activa el nuevo hilo
        }
    }

    pub fn sorteo(mut self) -> () {
        let mut random_number: u32 = rand::random();
        let mut aux_number: u32 = 0;
        if self.active_threads_aux > 0{
            let winner: u32 = random_number % (self.total_tickets + 1);//Ganador
            aux_number = winner;
            for mut i in 0..self.active_threads {//Revisa al ganador
                aux_number -= self.tickets[i as usize];
                if aux_number <= 0 {
                    if self.dead_threads[(i % self.active_threads) as usize] > 0 {
                        while self.dead_threads[(i % self.active_threads) as usize] > 0 {
                            i += 1;
                        }
                    }
                    self.curr_context = i;
                    //current_thread = &threads[self.curr_context];
                    break;
                }else{
                    self.tickets[i as usize] += 1;
                    self.total_tickets += 1;
                }
            }
            //setcontext(current_thread);//Activa el nuevo hilo
        }
    }
    
    pub fn real(mut self) -> () {
        /*int aux = -1;
        if(active_threads_aux > 0){
            int i;
            for (i = 0; i < active_threads; i++) {//Busca el hilo de mayor prioridad que aun no haya finalizado
                if(aux<priority[i]&&!deadThreads[i] && !priority_aux[i]){
                    curcontext = i;
                    aux = priority[i];
                }
            }
            if(aux == -1){
                for (i = 0; i < active_threads; i++) {
                    priority_aux[i] = 0;
                }
                real();
            }else{
                priority_aux[curcontext] = 1;//Hilo ya ejecutado
                current_thread = &threads[curcontext];
                setcontext(current_thread); //Activa el nuevo hilo
            }
        }*/
    }

    pub fn timer_interrupt(mut self) -> () {
        /*getcontext(&signal_context);
        signal_context.uc_stack.ss_sp = signal_stack;
        signal_context.uc_stack.ss_size = STACKSIZE;
        signal_context.uc_stack.ss_flags = 0;
        sigemptyset(&signal_context.uc_sigmask);
        //Si es de round robin
        if(active_sched == 0){
            makecontext(&signal_context, scheduler, 1);
        }
        //Si es de sorteo
        if(active_sched == 1){
            makecontext(&signal_context, sched_sorteo, 1);
        }
        //Si es de tiempo real
        if(active_sched == 2){
            makecontext(&signal_context, sched_real, 1);
        }
        //Cambia el context
        swapcontext(current_thread,&signal_context);*/
    }
}
