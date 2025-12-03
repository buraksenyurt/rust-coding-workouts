fn main() {
    let connection = DbConnection::new();
    let initialized_connection = connection.initialize("server=localhost;port=8080");
    match initialized_connection.connect() {
        Ok(_connected_connection) => {
            println!("Connection established successfully!");
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}

/*
    Durumları temsil eden tipler. Genellikle veri içermezler.
    Bunlar marker types olarak da bilinir.

    Aşağıdaki örnekte üç durum tanımlanmıştır:
    - Disconnected: Bağlantı kurulmamış durum
    - Initialized: Bağlantı başlatılmış ama henüz bağlanmamış durum
    - Connected: Bağlantı kurulmuş durum

    Initialized durumuna geçilebilmesi için önce Disconnected durumunda olunması gerekir.
    Connected durumuna geçilebilmesi için ise Initialized durumunda olunması gerekir.
*/
struct Disconnected;
struct Initialized;
struct Connected {
    _address: String,
}

// DbConnection yapısı, State tür parametresi ile durumunu belirtir.
struct DbConnection<State> {
    config: String,
    /*
        State türü, DbConnection yapısının bir parçası değildir ancak tür sistemi tarafından da izlenmesi gereken bir bilgidir.
        Bu nedenle PhantomData kullanılmakta. PhantomData, built-in bir marker type'dır.
        Derleme zamanında tür bilgisi olarak kullanılır, çalışma zamanında ise herhangi bir veri tutmaz.
        Zero Cost Abstraction (Sıfır Maliyetli Soyutlama) prensibine uygundur. 
        Bir başka deyişle, çalışma zamanında herhangi bir ek maliyet getirmez.
    */
    state: std::marker::PhantomData<State>,
}

impl DbConnection<Disconnected> {
    fn new() -> Self {
        println!("Creating new connection");
        DbConnection {
            config: String::new(),
            state: std::marker::PhantomData,
        }
    }

    fn initialize(mut self, config: &str) -> DbConnection<Initialized> {
        println!("Initializing connection with config: {}", config);
        self.config = config.to_string();

        DbConnection {
            config: self.config,
            state: std::marker::PhantomData,
        }
    }
}

impl DbConnection<Initialized> {
    fn connect(self) -> Result<DbConnection<Connected>, String> {
        println!("Connecting with config: {}", self.config);
        // Konfigürasyon geçerli ise ve bağlantı başarılı ise Connected durumuna geçiş yaparız.
        // Aksi halde hata döneriz. Burada basit bir örnek olması için her zaman başarılı sonuç dönüyoruz.
        Ok(DbConnection {
            config: self.config,
            state: std::marker::PhantomData,
        })
    }
}
