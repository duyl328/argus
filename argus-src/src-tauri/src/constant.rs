pub const BANNER1: &str = "

    // | |                               //   ) )   //
   //__| |     __      ___              ((         //
  / ___  |   //  ) ) //   ) ) //   / /    \\      //
 //    | |  //      ((___/ / //   / /       ) )
//     | | //        //__   ((___( ( ((___ / /  //     ";
pub const BANNER2: &str = "
         o                                                 o__ __o       o
        <|>                                               /v     v\\     <|>
        / \\                                              />       <\\    / \\
      o/   \\o       \\o__ __o     o__ __o/   o       o   _\\o____         \\o/
     <|__ __|>       |     |>   /v     |   <|>     <|>       \\_\\__o__    |
     /       \\      / \\   < >  />     / \\  < >     < >             \\    < >
   o/         \\o    \\o/        \\      \\o/   |       |    \\         /
  /v           v\\    |          o      |    o       o     o       o      o
 />             <\\  / \\         <\\__  < >   <\\__ __/>     <\\__ __/>    _<|>_
                                       |
                               o__     o
                               <\\__ __/>
";

pub const BANNER4: &str = "

               AAA                                                                           SSSSSSSSSSSSSSS  !!!
              A:::A                                                                        SS:::::::::::::::S!!:!!
             A:::::A                                                                      S:::::SSSSSS::::::S!:::!
            A:::::::A                                                                     S:::::S     SSSSSSS!:::!
           A:::::::::A          rrrrr   rrrrrrrrr      ggggggggg   ggggguuuuuu    uuuuuu  S:::::S            !:::!
          A:::::A:::::A         r::::rrr:::::::::r    g:::::::::ggg::::gu::::u    u::::u  S:::::S            !:::!
         A:::::A A:::::A        r:::::::::::::::::r  g:::::::::::::::::gu::::u    u::::u   S::::SSSS         !:::!
        A:::::A   A:::::A       rr::::::rrrrr::::::rg::::::ggggg::::::ggu::::u    u::::u    SS::::::SSSSS    !:::!
       A:::::A     A:::::A       r:::::r     r:::::rg:::::g     g:::::g u::::u    u::::u      SSS::::::::SS  !:::!
      A:::::AAAAAAAAA:::::A      r:::::r     rrrrrrrg:::::g     g:::::g u::::u    u::::u         SSSSSS::::S !:::!
     A:::::::::::::::::::::A     r:::::r            g:::::g     g:::::g u::::u    u::::u              S:::::S!!:!!
    A:::::AAAAAAAAAAAAA:::::A    r:::::r            g::::::g    g:::::g u:::::uuuu:::::u              S:::::S !!!
   A:::::A             A:::::A   r:::::r            g:::::::ggggg:::::g u:::::::::::::::uuSSSSSSS     S:::::S
  A:::::A               A:::::A  r:::::r             g::::::::::::::::g  u:::::::::::::::uS::::::SSSSSS:::::S !!!
 A:::::A                 A:::::A r:::::r              gg::::::::::::::g   uu::::::::uu:::uS:::::::::::::::SS !!:!!
AAAAAAA                   AAAAAAArrrrrrr                gggggggg::::::g     uuuuuuuu  uuuu SSSSSSSSSSSSSSS    !!!
                                                                g:::::g
                                                    gggggg      g:::::g
                                                    g:::::gg   gg:::::g
                                                     g::::::ggg:::::::g
                                                      gg:::::::::::::g
                                                        ggg::::::ggg
                                                           gggggg
";

pub const BANNER5: &str = "
              ,                                    _
            /'/                                  /' `\\       /'
          /' /                                 /'   ._)    /'
       ,/'  /     ____     ____               (____      /'
      /`--,/    )'    )--/'    )  /'    /          )   /'
    /'    /   /'       /'    /' /'    /'         /'  /'
(,/'     (_,/'        (___,/(__(___,/(__(_____,/'  O
                         /'
                 /     /'
                (___,/'
";
pub const BANNER6: &str = "
      ___           ___           ___           ___           ___
     /  /\\         /  /\\         /  /\\         /__/\\         /  /\\
    /  /::\\       /  /::\\       /  /:/_        \\  \\:\\       /  /:/_
   /  /:/\\:\\     /  /:/\\:\\     /  /:/ /\\        \\  \\:\\     /  /:/ /\\
  /  /:/~/::\\   /  /:/~/:/    /  /:/_/::\\   ___  \\  \\:\\   /  /:/ /::\\
 /__/:/ /:/\\:\\ /__/:/ /:/___ /__/:/__\\/\\:\\ /__/\\  \\__\\:\\ /__/:/ /:/\\:\\
 \\  \\:\\/:/__\\/ \\  \\:\\/:::::/ \\  \\:\\ /~~/:/ \\  \\:\\ /  /:/ \\  \\:\\/:/~/:/
  \\  \\::/       \\  \\::/~~~~   \\  \\:\\  /:/   \\  \\:\\  /:/   \\  \\::/ /:/
   \\  \\:\\        \\  \\:\\        \\  \\:\\/:/     \\  \\:\\/:/     \\__\\/ /:/
    \\  \\:\\        \\  \\:\\        \\  \\::/       \\  \\::/        /__/:/
     \\__\\/         \\__\\/         \\__\\/         \\__\\/         \\__\\/
";

/// 数据库链接
pub const DATABASE_URL_KEY: &str = "DATABASE_URL";

/// 数据库默认连接
pub const DATABASE_DEFAULT_LINK: &str = "db/sqlite.db";

/// 数据库名称
pub const DATABASE_NAME: &str = "sqlite.db";
pub const DATABASE_PATH: &str = "db";

/// 日志输出路径
pub const LOG_PATH: &str = "/path/to/tauri-logs";

/// 当前数据库版本
pub const CURRENT_DB_VERSION: u32 = 1;

/// 默认 `db_version` 元素的 `id` 因为只能由一个，ID 唯一
pub const BASE_DB_VERSION_ITEM_ID: u32 = 1;

/// 时间默认格式
pub const TIME_BASIC_FMT: &str = "%Y-%m-%d %H:%M:%S";

/// 基础设置 ID 
pub const BASIC_SETTING_ID: i32 = 1;