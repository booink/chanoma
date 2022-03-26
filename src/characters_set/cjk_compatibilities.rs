use crate::corr::{Corr, Correspondence, Item, Linear};

#[derive(Clone)]
pub struct CjkCompatibilities;

impl Corr for CjkCompatibilities {
    fn items(&self) -> Vec<Item> {
        (SQUARE_APAATO
            + &SQUARE_ARUHUA
            + &SQUARE_ANPEA
            + &SQUARE_AARU
            + &SQUARE_ININGU
            + &SQUARE_INTI
            + &SQUARE_UON
            + &SQUARE_ESUKUUDO
            + &SQUARE_EEKAA
            + &SQUARE_ONSU
            + &SQUARE_OOMU
            + &SQUARE_KAIRI
            + &SQUARE_KARATTO
            + &SQUARE_KARORII
            + &SQUARE_GARON
            + &SQUARE_GANMA
            + &SQUARE_GIGA
            + &SQUARE_GINII
            + &SQUARE_KYURII
            + &SQUARE_GIRUDAA
            + &SQUARE_KIRO
            + &SQUARE_KIROGURAMU
            + &SQUARE_KIROMEETORU
            + &SQUARE_KIROWATTO
            + &SQUARE_GURAMU
            + &SQUARE_GURAMUTON
            + &SQUARE_KURUZEIRO
            + &SQUARE_KUROONE
            + &SQUARE_KEESU
            + &SQUARE_KORUNA
            + &SQUARE_KOOPO
            + &SQUARE_SAIKURU
            + &SQUARE_SANTIIMU
            + &SQUARE_SIRINGU
            + &SQUARE_SENTI
            + &SQUARE_SENTO
            + &SQUARE_DAASU
            + &SQUARE_DESI
            + &SQUARE_DORU
            + &SQUARE_TON
            + &SQUARE_NANO
            + &SQUARE_NOTTO
            + &SQUARE_HAITU
            + &SQUARE_PAASENTO
            + &SQUARE_PAATU
            + &SQUARE_BAARERU
            + &SQUARE_PIASUTORU
            + &SQUARE_PIKURU
            + &SQUARE_PIKO
            + &SQUARE_BIRU
            + &SQUARE_HUARADDO
            + &SQUARE_HUIITO
            + &SQUARE_BUSSYERU
            + &SQUARE_HURAN
            + &SQUARE_HEKUTAARU
            + &SQUARE_PESO
            + &SQUARE_PENIHI
            + &SQUARE_HERUTU
            + &SQUARE_PENSU
            + &SQUARE_PEEZI
            + &SQUARE_BEETA
            + &SQUARE_POINTO
            + &SQUARE_BORUTO
            + &SQUARE_HON
            + &SQUARE_PONDO
            + &SQUARE_HOORU
            + &SQUARE_HOON
            + &SQUARE_MAIKURO
            + &SQUARE_MAIRU
            + &SQUARE_MAHHA
            + &SQUARE_MARUKU
            + &SQUARE_MANSYON
            + &SQUARE_MIKURON
            + &SQUARE_MIRI
            + &SQUARE_MIRIBAARU
            + &SQUARE_MEGA
            + &SQUARE_MEGATON
            + &SQUARE_MEETORU
            + &SQUARE_YAADO
            + &SQUARE_YAARU
            + &SQUARE_YUAN
            + &SQUARE_RITTORU
            + &SQUARE_RIRA
            + &SQUARE_RUPII
            + &SQUARE_RUUBURU
            + &SQUARE_REMU
            + &SQUARE_RENTOGEN
            + &SQUARE_WATTO
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_ZERO
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_ONE
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_TWO
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_THREE
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_FOUR
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_FIVE
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_SIX
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_SEVEN
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_EIGHT
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_NINE
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_TEN
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_ELEVEN
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_TWELVE
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_THIRTEEN
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_FOURTEEN
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_FIFTEEN
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_SIXTEEN
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_SEVENTEEN
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_EIGHTEEN
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_NINETEEN
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_TWENTY
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_TWENTY_ONE
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_TWENTY_TWO
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_TWENTY_THREE
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_TWENTY_FOUR
            + &SQUARE_HPA
            + &SQUARE_DA
            + &SQUARE_AU
            + &SQUARE_BAR
            + &SQUARE_OV
            + &SQUARE_PC
            + &SQUARE_DM
            + &SQUARE_DM_SQUARED
            + &SQUARE_DM_CUBED
            + &SQUARE_IU
            + &SQUARE_ERA_NAME_HEISEI
            + &SQUARE_ERA_NAME_SYOUWA
            + &SQUARE_ERA_NAME_TAISYOU
            + &SQUARE_ERA_NAME_MEIZI
            + &SQUARE_CORPORATION
            + &SQUARE_PA_AMPS
            + &SQUARE_NA
            + &SQUARE_MU_A
            + &SQUARE_MA
            + &SQUARE_KA
            + &SQUARE_KB
            + &SQUARE_MB
            + &SQUARE_GB
            + &SQUARE_CAL
            + &SQUARE_KCAL
            + &SQUARE_PF
            + &SQUARE_NF
            + &SQUARE_MU_F
            + &SQUARE_MU_G
            + &SQUARE_MG
            + &SQUARE_KG
            + &SQUARE_HZ
            + &SQUARE_KHZ
            + &SQUARE_MHZ
            + &SQUARE_GHZ
            + &SQUARE_THZ
            + &SQUARE_MU_L
            + &SQUARE_ML
            + &SQUARE_DL
            + &SQUARE_KL
            + &SQUARE_FM
            + &SQUARE_NM
            + &SQUARE_MU_M
            + &SQUARE_MM
            + &SQUARE_CM
            + &SQUARE_KM
            + &SQUARE_MM_SQUARED
            + &SQUARE_CM_SQUARED
            + &SQUARE_M_SQUARED
            + &SQUARE_KM_SQUARED
            + &SQUARE_MM_CUBED
            + &SQUARE_CM_CUBED
            + &SQUARE_M_CUBED
            + &SQUARE_KM_CUBED
            + &SQUARE_M_OVER_S
            + &SQUARE_M_OVER_S_SQUARED
            + &SQUARE_PA
            + &SQUARE_KPA
            + &SQUARE_MPA
            + &SQUARE_GPA
            + &SQUARE_RAD
            + &SQUARE_RAD_OVER_S
            + &SQUARE_RAD_OVER_S_SQUARED
            + &SQUARE_PS
            + &SQUARE_NS
            + &SQUARE_MU_S
            + &SQUARE_MS
            + &SQUARE_PV
            + &SQUARE_NV
            + &SQUARE_MU_V
            + &SQUARE_MV
            + &SQUARE_KV
            + &SQUARE_MV_MEGA
            + &SQUARE_PW
            + &SQUARE_NW
            + &SQUARE_MU_W
            + &SQUARE_MW
            + &SQUARE_KW
            + &SQUARE_MW_MEGA
            + &SQUARE_K_OHM
            + &SQUARE_M_OHM
            + &SQUARE_AM
            + &SQUARE_BQ
            + &SQUARE_CC
            + &SQUARE_CD
            + &SQUARE_C_OVER_KG
            + &SQUARE_CO
            + &SQUARE_DB
            + &SQUARE_GY
            + &SQUARE_HA
            + &SQUARE_HP
            + &SQUARE_IN
            + &SQUARE_KK
            + &SQUARE_KM_CAPITAL
            + &SQUARE_KT
            + &SQUARE_LM
            + &SQUARE_LN
            + &SQUARE_LOG
            + &SQUARE_LX
            + &SQUARE_MB_SMALL
            + &SQUARE_MIL
            + &SQUARE_MOL
            + &SQUARE_PH
            + &SQUARE_PM
            + &SQUARE_PPM
            + &SQUARE_PR
            + &SQUARE_SR
            + &SQUARE_SV
            + &SQUARE_WB
            + &SQUARE_V_OVER_M
            + &SQUARE_A_OVER_M
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_ONE
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_TWO
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_THREE
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_FOUR
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_FIVE
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_SIX
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_SEVEN
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_EIGHT
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_NINE
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_TEN
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_ELEVEN
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_TWELVE
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_THIRTEEN
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_FOURTEEN
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_FIFTEEN
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_SIXTEEN
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_SEVENTEEN
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_EIGHTEEN
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_NINETEEN
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_TWENTY
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_TWENTY_ONE
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_TWENTY_TWO
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_TWENTY_THREE
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_TWENTY_FOUR
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_TWENTY_FIVE
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_TWENTY_SIX
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_TWENTY_SEVEN
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_TWENTY_EIGHT
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_TWENTY_NINE
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_THIRTY
            + &IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_THIRTY_ONE
            + &SQUARE_GAL)
            .items()
    }
}

impl CjkCompatibilities {
    pub const fn new() -> Self {
        Self {}
    }

    pub const fn corr(self) -> Correspondence<Self> {
        Correspondence::new(self)
    }
}

// CJK互換用文字
pub const SQUARE_APAATO: Correspondence<Linear> = Linear::new("\u{3300}", "アパート").corr(); // U+3300  ㌀  SQUARE APAATO   アパート
pub const SQUARE_ARUHUA: Correspondence<Linear> = Linear::new("\u{3301}", "アルファ").corr(); // U+3301  ㌁  SQUARE ARUHUA   アルファ
pub const SQUARE_ANPEA: Correspondence<Linear> = Linear::new("\u{3302}", "アンペア").corr(); // U+3302  ㌂  SQUARE ANPEA    アンペア
pub const SQUARE_AARU: Correspondence<Linear> = Linear::new("\u{3303}", "アール").corr(); // U+3303  ㌃  SQUARE AARU     アール
pub const SQUARE_ININGU: Correspondence<Linear> = Linear::new("\u{3304}", "イニング").corr(); // U+3304  ㌄  SQUARE ININGU   イニング
pub const SQUARE_INTI: Correspondence<Linear> = Linear::new("\u{3305}", "インチ").corr(); // U+3305  ㌅  SQUARE INTI     インチ
pub const SQUARE_UON: Correspondence<Linear> = Linear::new("\u{3306}", "ウォン").corr(); // U+3306  ㌆  SQUARE UON  ウォン
pub const SQUARE_ESUKUUDO: Correspondence<Linear> = Linear::new("\u{3307}", "エスクード").corr(); // U+3307  ㌇  SQUARE ESUKUUDO     エスクード
pub const SQUARE_EEKAA: Correspondence<Linear> = Linear::new("\u{3308}", "エーカー").corr(); // U+3308  ㌈  SQUARE EEKAA    エーカー
pub const SQUARE_ONSU: Correspondence<Linear> = Linear::new("\u{3309}", "オンス").corr(); // U+3309  ㌉  SQUARE ONSU     オンス
pub const SQUARE_OOMU: Correspondence<Linear> = Linear::new("\u{330A}", "オーム").corr(); // U+330A  ㌊  SQUARE OOMU     オーム
pub const SQUARE_KAIRI: Correspondence<Linear> = Linear::new("\u{330B}", "カイリ").corr(); // U+330B  ㌋  SQUARE KAIRI    カイリ
pub const SQUARE_KARATTO: Correspondence<Linear> = Linear::new("\u{330C}", "カラット").corr(); // U+330C  ㌌  SQUARE KARATTO  カラット
pub const SQUARE_KARORII: Correspondence<Linear> = Linear::new("\u{330D}", "カロリー").corr(); // U+330D  ㌍  SQUARE KARORII  カロリー
pub const SQUARE_GARON: Correspondence<Linear> = Linear::new("\u{330E}", "ガロン").corr(); // U+330E  ㌎  SQUARE GARON    ガロン
pub const SQUARE_GANMA: Correspondence<Linear> = Linear::new("\u{330F}", "ガンマ").corr(); // U+330F  ㌏  SQUARE GANMA    ガンマ  γ（ギリシャ文字
pub const SQUARE_GIGA: Correspondence<Linear> = Linear::new("\u{3310}", "ギガ").corr(); // U+3310  ㌐  SQUARE GIGA     ギガ
pub const SQUARE_GINII: Correspondence<Linear> = Linear::new("\u{3311}", "ギニー").corr(); // U+3311  ㌑  SQUARE GINII    ギニー
pub const SQUARE_KYURII: Correspondence<Linear> = Linear::new("\u{3312}", "キュリー").corr(); // U+3312  ㌒  SQUARE KYURII   キュリー
pub const SQUARE_GIRUDAA: Correspondence<Linear> = Linear::new("\u{3313}", "ギルダー").corr(); // U+3313  ㌓  SQUARE GIRUDAA  ギルダー
pub const SQUARE_KIRO: Correspondence<Linear> = Linear::new("\u{3314}", "キロ").corr(); // U+3314  ㌔  SQUARE KIRO     キロ
pub const SQUARE_KIROGURAMU: Correspondence<Linear> = Linear::new("\u{3315}", "キログラム").corr(); // U+3315  ㌕  SQUARE KIROGURAMU   キログラム
pub const SQUARE_KIROMEETORU: Correspondence<Linear> =
    Linear::new("\u{3316}", "キロメートル").corr(); // U+3316  ㌖  SQUARE KIROMEETORU  キロメートル
pub const SQUARE_KIROWATTO: Correspondence<Linear> = Linear::new("\u{3317}", "キロワット").corr(); // U+3317  ㌗  SQUARE KIROWATTO    キロワット
pub const SQUARE_GURAMU: Correspondence<Linear> = Linear::new("\u{3318}", "グラム").corr(); // U+3318  ㌘  SQUARE GURAMU   グラム
pub const SQUARE_GURAMUTON: Correspondence<Linear> = Linear::new("\u{3319}", "グラムトン").corr(); // U+3319  ㌙  SQUARE GURAMUTON    グラムトン
pub const SQUARE_KURUZEIRO: Correspondence<Linear> = Linear::new("\u{331A}", "クルゼイロ").corr(); // U+331A  ㌚  SQUARE KURUZEIRO    クルゼイロ
pub const SQUARE_KUROONE: Correspondence<Linear> = Linear::new("\u{331B}", "クローネ").corr(); // U+331B  ㌛  SQUARE KUROONE  クローネ
pub const SQUARE_KEESU: Correspondence<Linear> = Linear::new("\u{331C}", "ケース").corr(); // U+331C  ㌜  SQUARE KEESU    ケース  ケース
pub const SQUARE_KORUNA: Correspondence<Linear> = Linear::new("\u{331D}", "コルナ").corr(); // U+331D  ㌝  SQUARE KORUNA   コルナ
pub const SQUARE_KOOPO: Correspondence<Linear> = Linear::new("\u{331E}", "コーポ").corr(); // U+331E  ㌞  SQUARE KOOPO    コーポ
pub const SQUARE_SAIKURU: Correspondence<Linear> = Linear::new("\u{331F}", "サイクル").corr(); // U+331F  ㌟  SQUARE SAIKURU  サイクル
pub const SQUARE_SANTIIMU: Correspondence<Linear> = Linear::new("\u{3320}", "サンチーム").corr(); // U+3320  ㌠  SQUARE SANTIIMU     サンチーム
pub const SQUARE_SIRINGU: Correspondence<Linear> = Linear::new("\u{3321}", "シリング").corr(); // U+3321  ㌡  SQUARE SIRINGU  シリング
pub const SQUARE_SENTI: Correspondence<Linear> = Linear::new("\u{3322}", "センチ").corr(); // U+3322  ㌢  SQUARE SENTI    センチ
pub const SQUARE_SENTO: Correspondence<Linear> = Linear::new("\u{3323}", "セント").corr(); // U+3323  ㌣  SQUARE SENTO    セント
pub const SQUARE_DAASU: Correspondence<Linear> = Linear::new("\u{3324}", "ダース").corr(); // U+3324  ㌤  SQUARE DAASU    ダース
pub const SQUARE_DESI: Correspondence<Linear> = Linear::new("\u{3325}", "デシ").corr(); // U+3325  ㌥  SQUARE DESI     デシ
pub const SQUARE_DORU: Correspondence<Linear> = Linear::new("\u{3326}", "ドル").corr(); // U+3326  ㌦  SQUARE DORU     ドル
pub const SQUARE_TON: Correspondence<Linear> = Linear::new("\u{3327}", "トン").corr(); // U+3327  ㌧  SQUARE TON  トン
pub const SQUARE_NANO: Correspondence<Linear> = Linear::new("\u{3328}", "ナノ").corr(); // U+3328  ㌨  SQUARE NANO     ナノ
pub const SQUARE_NOTTO: Correspondence<Linear> = Linear::new("\u{3329}", "ノット").corr(); // U+3329  ㌩  SQUARE NOTTO    ノット
pub const SQUARE_HAITU: Correspondence<Linear> = Linear::new("\u{332A}", "ハイツ").corr(); // U+332A  ㌪  SQUARE HAITU    ハイツ
pub const SQUARE_PAASENTO: Correspondence<Linear> = Linear::new("\u{332B}", "パーセント").corr(); // U+332B  ㌫  SQUARE PAASENTO     パーセント
pub const SQUARE_PAATU: Correspondence<Linear> = Linear::new("\u{332C}", "パーツ").corr(); // U+332C  ㌬  SQUARE PAATU    パーツ  パーツ
pub const SQUARE_BAARERU: Correspondence<Linear> = Linear::new("\u{332D}", "バーレル").corr(); // U+332D  ㌭  SQUARE BAARERU  バーレル
pub const SQUARE_PIASUTORU: Correspondence<Linear> = Linear::new("\u{332E}", "ピアストル").corr(); // U+332E  ㌮  SQUARE PIASUTORU    ピアストル
pub const SQUARE_PIKURU: Correspondence<Linear> = Linear::new("\u{332F}", "ピクル").corr(); // U+332F  ㌯  SQUARE PIKURU   ピクル  ピクル（担の別名
pub const SQUARE_PIKO: Correspondence<Linear> = Linear::new("\u{3330}", "ピコ").corr(); // U+3330  ㌰  SQUARE PIKO     ピコ
pub const SQUARE_BIRU: Correspondence<Linear> = Linear::new("\u{3331}", "ビル").corr(); // U+3331  ㌱  SQUARE BIRU     ビル
pub const SQUARE_HUARADDO: Correspondence<Linear> = Linear::new("\u{3332}", "ファラッド").corr(); // U+3332  ㌲  SQUARE HUARADDO     ファラッド
pub const SQUARE_HUIITO: Correspondence<Linear> = Linear::new("\u{3333}", "フィート").corr(); // U+3333  ㌳  SQUARE HUIITO   フィート
pub const SQUARE_BUSSYERU: Correspondence<Linear> = Linear::new("\u{3334}", "ブッシェル").corr(); // U+3334  ㌴  SQUARE BUSSYERU     ブッシェル
pub const SQUARE_HURAN: Correspondence<Linear> = Linear::new("\u{3335}", "フラン").corr(); // U+3335  ㌵  SQUARE HURAN    フラン
pub const SQUARE_HEKUTAARU: Correspondence<Linear> = Linear::new("\u{3336}", "ヘクタール").corr(); // U+3336  ㌶  SQUARE HEKUTAARU    ヘクタール
pub const SQUARE_PESO: Correspondence<Linear> = Linear::new("\u{3337}", "ペソ").corr(); // U+3337  ㌷  SQUARE PESO     ペソ
pub const SQUARE_PENIHI: Correspondence<Linear> = Linear::new("\u{3338}", "ペニヒ").corr(); // U+3338  ㌸  SQUARE PENIHI   ペニヒ
pub const SQUARE_HERUTU: Correspondence<Linear> = Linear::new("\u{3339}", "ヘルツ").corr(); // U+3339  ㌹  SQUARE HERUTU   ヘルツ
pub const SQUARE_PENSU: Correspondence<Linear> = Linear::new("\u{333A}", "ペンス").corr(); // U+333A  ㌺  SQUARE PENSU    ペンス
pub const SQUARE_PEEZI: Correspondence<Linear> = Linear::new("\u{333B}", "ページ").corr(); // U+333B  ㌻  SQUARE PEEZI    ページ
pub const SQUARE_BEETA: Correspondence<Linear> = Linear::new("\u{333C}", "ベータ").corr(); // U+333C  ㌼  SQUARE BEETA    ベータ  β（ギリシャ文字
pub const SQUARE_POINTO: Correspondence<Linear> = Linear::new("\u{333D}", "ポイント").corr(); // U+333D  ㌽  SQUARE POINTO   ポイント
pub const SQUARE_BORUTO: Correspondence<Linear> = Linear::new("\u{333E}", "ボルト").corr(); // U+333E  ㌾  SQUARE BORUTO   ボルト
pub const SQUARE_HON: Correspondence<Linear> = Linear::new("\u{333F}", "ホン").corr(); // U+333F  ㌿  SQUARE HON  ホン
pub const SQUARE_PONDO: Correspondence<Linear> = Linear::new("\u{3340}", "ポンド").corr(); // U+3340  ㍀  SQUARE PONDO    ポンド
pub const SQUARE_HOORU: Correspondence<Linear> = Linear::new("\u{3341}", "ホール").corr(); // U+3341  ㍁  SQUARE HOORU    ホール  ホール
pub const SQUARE_HOON: Correspondence<Linear> = Linear::new("\u{3342}", "ホーン").corr(); // U+3342  ㍂  SQUARE HOON     ホーン
pub const SQUARE_MAIKURO: Correspondence<Linear> = Linear::new("\u{3343}", "マイクロ").corr(); // U+3343  ㍃  SQUARE MAIKURO  マイクロ
pub const SQUARE_MAIRU: Correspondence<Linear> = Linear::new("\u{3344}", "マイル").corr(); // U+3344  ㍄  SQUARE MAIRU    マイル
pub const SQUARE_MAHHA: Correspondence<Linear> = Linear::new("\u{3345}", "マッハ").corr(); // U+3345  ㍅  SQUARE MAHHA    マッハ
pub const SQUARE_MARUKU: Correspondence<Linear> = Linear::new("\u{3346}", "マルク").corr(); // U+3346  ㍆  SQUARE MARUKU   マルク
pub const SQUARE_MANSYON: Correspondence<Linear> = Linear::new("\u{3347}", "マンション").corr(); // U+3347  ㍇  SQUARE MANSYON  マンション
pub const SQUARE_MIKURON: Correspondence<Linear> = Linear::new("\u{3348}", "ミクロン").corr(); // U+3348  ㍈  SQUARE MIKURON  ミクロン
pub const SQUARE_MIRI: Correspondence<Linear> = Linear::new("\u{3349}", "ミリ").corr(); // U+3349  ㍉  SQUARE MIRI     ミリ
pub const SQUARE_MIRIBAARU: Correspondence<Linear> = Linear::new("\u{334A}", "ミリバール").corr(); // U+334A  ㍊  SQUARE MIRIBAARU    ミリバール
pub const SQUARE_MEGA: Correspondence<Linear> = Linear::new("\u{334B}", "メガ").corr(); // U+334B  ㍋  SQUARE MEGA     メガ
pub const SQUARE_MEGATON: Correspondence<Linear> = Linear::new("\u{334C}", "メガトン").corr(); // U+334C  ㍌  SQUARE MEGATON  メガトン
pub const SQUARE_MEETORU: Correspondence<Linear> = Linear::new("\u{334D}", "メートル").corr(); // U+334D  ㍍  SQUARE MEETORU  メートル
pub const SQUARE_YAADO: Correspondence<Linear> = Linear::new("\u{334E}", "ヤード").corr(); // U+334E  ㍎  SQUARE YAADO    ヤード
pub const SQUARE_YAARU: Correspondence<Linear> = Linear::new("\u{334F}", "ヤール").corr(); // U+334F  ㍏  SQUARE YAARU    ヤール  ヤール（ヤードの別名
pub const SQUARE_YUAN: Correspondence<Linear> = Linear::new("\u{3350}", "ユアン").corr(); // U+3350  ㍐  SQUARE YUAN     ユアン  ユアン（元の英語名
pub const SQUARE_RITTORU: Correspondence<Linear> = Linear::new("\u{3351}", "リットル").corr(); // U+3351  ㍑  SQUARE RITTORU  リットル
pub const SQUARE_RIRA: Correspondence<Linear> = Linear::new("\u{3352}", "リラ").corr(); // U+3352  ㍒  SQUARE RIRA     リラ
pub const SQUARE_RUPII: Correspondence<Linear> = Linear::new("\u{3353}", "ルピー").corr(); // U+3353  ㍓  SQUARE RUPII    ルピー
pub const SQUARE_RUUBURU: Correspondence<Linear> = Linear::new("\u{3354}", "ルーブル").corr(); // U+3354  ㍔  SQUARE RUUBURU  ルーブル
pub const SQUARE_REMU: Correspondence<Linear> = Linear::new("\u{3355}", "レム").corr(); // U+3355  ㍕  SQUARE REMU     レム
pub const SQUARE_RENTOGEN: Correspondence<Linear> = Linear::new("\u{3356}", "レントゲン").corr(); // U+3356  ㍖  SQUARE RENTOGEN     レントゲン
pub const SQUARE_WATTO: Correspondence<Linear> = Linear::new("\u{3357}", "ワット").corr(); // U+3357  ㍗  SQUARE WATTO    ワット
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_ZERO: Correspondence<Linear> =
    Linear::new("\u{3358}", "0点").corr(); // U+3358  ㍘  IDEOGRAPHIC TELEGRAPH SYMBOL FOR HOUR ZERO  0点     0点（中国語で0時の意）の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_ONE: Correspondence<Linear> =
    Linear::new("\u{3359}", "1点").corr(); // U+3359  ㍙  IDEOGRAPHIC TELEGRAPH SYMBOL FOR HOUR ONE   1点     1点（中国語で1時の意）の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_TWO: Correspondence<Linear> =
    Linear::new("\u{335A}", "2点").corr(); // U+335A  ㍚  IDEOGRAPHIC TELEGRAPH SYMBOL FOR HOUR TWO   2点     2点（中国語で2時の意）の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_THREE: Correspondence<Linear> =
    Linear::new("\u{335B}", "3点").corr(); // U+335B  ㍛  IDEOGRAPHIC TELEGRAPH SYMBOL FOR HOUR THREE     3点     3点（中国語で3時の意）の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_FOUR: Correspondence<Linear> =
    Linear::new("\u{335C}", "4点").corr(); // U+335C  ㍜  IDEOGRAPHIC TELEGRAPH SYMBOL FOR HOUR FOUR  4点     4点（中国語で4時の意）の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_FIVE: Correspondence<Linear> =
    Linear::new("\u{335D}", "5点").corr(); // U+335D  ㍝  IDEOGRAPHIC TELEGRAPH SYMBOL FOR HOUR FIVE  5点     5点（中国語で5時の意）の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_SIX: Correspondence<Linear> =
    Linear::new("\u{335E}", "6点").corr(); // U+335E  ㍞  IDEOGRAPHIC TELEGRAPH SYMBOL FOR HOUR SIX   6点     6点（中国語で6時の意）の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_SEVEN: Correspondence<Linear> =
    Linear::new("\u{335F}", "7点").corr(); // U+335F  ㍟  IDEOGRAPHIC TELEGRAPH SYMBOL FOR HOUR SEVEN     7点     7点（中国語で7時の意）の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_EIGHT: Correspondence<Linear> =
    Linear::new("\u{3360}", "8点").corr(); // U+3360  ㍠  IDEOGRAPHIC TELEGRAPH SYMBOL FOR HOUR EIGHT     8点     8点（中国語で8時の意）の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_NINE: Correspondence<Linear> =
    Linear::new("\u{3361}", "9点").corr(); // U+3361  ㍡  IDEOGRAPHIC TELEGRAPH SYMBOL FOR HOUR NINE  9点     9点（中国語で9時の意）の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_TEN: Correspondence<Linear> =
    Linear::new("\u{3362}", "10点").corr(); // U+3362  ㍢  IDEOGRAPHIC TELEGRAPH SYMBOL FOR HOUR TEN   10点    10点（中国語で10時の意）の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_ELEVEN: Correspondence<Linear> =
    Linear::new("\u{3363}", "11点").corr(); // U+3363  ㍣  IDEOGRAPHIC TELEGRAPH SYMBOL FOR HOUR ELEVEN    11点    11点（中国語で11時の意）の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_TWELVE: Correspondence<Linear> =
    Linear::new("\u{3364}", "12点").corr(); // U+3364  ㍤  IDEOGRAPHIC TELEGRAPH SYMBOL FOR HOUR TWELVE    12点    12点（中国語で12時の意）の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_THIRTEEN: Correspondence<Linear> =
    Linear::new("\u{3365}", "13点").corr(); // U+3365  ㍥  IDEOGRAPHIC TELEGRAPH SYMBOL FOR HOUR THIRTEEN  13点    13点（中国語で13時の意）の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_FOURTEEN: Correspondence<Linear> =
    Linear::new("\u{3366}", "14点").corr(); // U+3366  ㍦  IDEOGRAPHIC TELEGRAPH SYMBOL FOR HOUR FOURTEEN  14点    14点（中国語で14時の意）の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_FIFTEEN: Correspondence<Linear> =
    Linear::new("\u{3367}", "15点").corr(); // U+3367  ㍧  IDEOGRAPHIC TELEGRAPH SYMBOL FOR HOUR FIFTEEN   15点    15点（中国語で15時の意）の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_SIXTEEN: Correspondence<Linear> =
    Linear::new("\u{3368}", "16点").corr(); // U+3368  ㍨  IDEOGRAPHIC TELEGRAPH SYMBOL FOR HOUR SIXTEEN   16点    16点（中国語で16時の意）の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_SEVENTEEN: Correspondence<Linear> =
    Linear::new("\u{3369}", "17点").corr(); // U+3369  ㍩  IDEOGRAPHIC TELEGRAPH SYMBOL FOR HOUR SEVENTEEN     17点    17点（中国語で17時の意）の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_EIGHTEEN: Correspondence<Linear> =
    Linear::new("\u{336A}", "18点").corr(); // U+336A  ㍪  IDEOGRAPHIC TELEGRAPH SYMBOL FOR HOUR EIGHTEEN  18点    18点（中国語で18時の意）の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_NINETEEN: Correspondence<Linear> =
    Linear::new("\u{336B}", "19点").corr(); // U+336B  ㍫  IDEOGRAPHIC TELEGRAPH SYMBOL FOR HOUR NINETEEN  19点    19点（中国語で19時の意）の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_TWENTY: Correspondence<Linear> =
    Linear::new("\u{336C}", "20点").corr(); // U+336C  ㍬  IDEOGRAPHIC TELEGRAPH SYMBOL FOR HOUR TWENTY    20点    20点（中国語で20時の意）の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_TWENTY_ONE: Correspondence<Linear> =
    Linear::new("\u{336D}", "21点").corr(); // U+336D  ㍭  IDEOGRAPHIC TELEGRAPH SYMBOL FOR HOUR TWENTY-ONE    21点    21点（中国語で21時の意）の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_TWENTY_TWO: Correspondence<Linear> =
    Linear::new("\u{336E}", "22点").corr(); // U+336E  ㍮  IDEOGRAPHIC TELEGRAPH SYMBOL FOR HOUR TWENTY-TWO    22点    22点（中国語で22時の意）の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_TWENTY_THREE: Correspondence<Linear> =
    Linear::new("\u{336F}", "23点").corr(); // U+336F  ㍯  IDEOGRAPHIC TELEGRAPH SYMBOL FOR HOUR TWENTY-THREE  23点    23点（中国語で23時の意）の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_HOUR_TWENTY_FOUR: Correspondence<Linear> =
    Linear::new("\u{3370}", "24点").corr(); // U+3370  ㍰  IDEOGRAPHIC TELEGRAPH SYMBOL FOR HOUR TWENTY-FOUR   24点    24点（中国語で24時の意）の漢字電信記号
pub const SQUARE_HPA: Correspondence<Linear> = Linear::new("\u{3371}", "hPa").corr(); // U+3371  ㍱  SQUARE HPA  hPa
pub const SQUARE_DA: Correspondence<Linear> = Linear::new("\u{3372}", "da").corr(); // U+3372  ㍲  SQUARE DA   da
pub const SQUARE_AU: Correspondence<Linear> = Linear::new("\u{3373}", "AU").corr(); // U+3373  ㍳  SQUARE AU   AU
pub const SQUARE_BAR: Correspondence<Linear> = Linear::new("\u{3374}", "bar").corr(); // U+3374  ㍴  SQUARE BAR  bar     気圧
pub const SQUARE_OV: Correspondence<Linear> = Linear::new("\u{3375}", "oV").corr(); // U+3375  ㍵  SQUARE OV   oV  基礎体温（単位、35.5～38.0度を50等分した
pub const SQUARE_PC: Correspondence<Linear> = Linear::new("\u{3376}", "pc").corr(); // U+3376  ㍶  SQUARE PC   pc
pub const SQUARE_DM: Correspondence<Linear> = Linear::new("\u{3377}", "dm").corr(); // U+3377  ㍷  SQUARE DM   dm
pub const SQUARE_DM_SQUARED: Correspondence<Linear> = Linear::new("\u{3378}", "dm2").corr(); // U+3378  ㍸  SQUARE DM SQUARED   dm2     平方デシメートル
pub const SQUARE_DM_CUBED: Correspondence<Linear> = Linear::new("\u{3379}", "dm3").corr(); // U+3379  ㍹  SQUARE DM CUBED     dm3     立方デシメートル
pub const SQUARE_IU: Correspondence<Linear> = Linear::new("\u{337A}", "IU").corr(); // U+337A  ㍺  SQUARE IU   IU
pub const SQUARE_ERA_NAME_HEISEI: Correspondence<Linear> = Linear::new("\u{337B}", "平成").corr(); // U+337B  ㍻  SQUARE ERA NAME HEISEI  平成
pub const SQUARE_ERA_NAME_SYOUWA: Correspondence<Linear> = Linear::new("\u{337C}", "昭和").corr(); // U+337C  ㍼  SQUARE ERA NAME SYOUWA  昭和
pub const SQUARE_ERA_NAME_TAISYOU: Correspondence<Linear> = Linear::new("\u{337D}", "大正").corr(); // U+337D  ㍽  SQUARE ERA NAME TAISYOU     大正
pub const SQUARE_ERA_NAME_MEIZI: Correspondence<Linear> = Linear::new("\u{337E}", "明治").corr(); // U+337E  ㍾  SQUARE ERA NAME MEIZI   明治
pub const SQUARE_CORPORATION: Correspondence<Linear> = Linear::new("\u{337F}", "株式会社").corr(); // U+337F  ㍿  SQUARE CORPORATION  株式会社
pub const SQUARE_PA_AMPS: Correspondence<Linear> = Linear::new("\u{3380}", "pA").corr(); // U+3380  ㎀  SQUARE PA AMPS  p
pub const SQUARE_NA: Correspondence<Linear> = Linear::new("\u{3381}", "nA").corr(); // U+3381  ㎁  SQUARE NA   n
pub const SQUARE_MU_A: Correspondence<Linear> = Linear::new("\u{3382}", "µA").corr(); // U+3382  ㎂  SQUARE MU A     µA
pub const SQUARE_MA: Correspondence<Linear> = Linear::new("\u{3383}", "mA").corr(); // U+3383  ㎃  SQUARE MA   m
pub const SQUARE_KA: Correspondence<Linear> = Linear::new("\u{3384}", "kA").corr(); // U+3384  ㎄  SQUARE KA   k
pub const SQUARE_KB: Correspondence<Linear> = Linear::new("\u{3385}", "KB").corr(); // U+3385  ㎅  SQUARE KB   KB
pub const SQUARE_MB: Correspondence<Linear> = Linear::new("\u{3386}", "MB").corr(); // U+3386  ㎆  SQUARE MB   MB
pub const SQUARE_GB: Correspondence<Linear> = Linear::new("\u{3387}", "GB").corr(); // U+3387  ㎇  SQUARE GB   GB
pub const SQUARE_CAL: Correspondence<Linear> = Linear::new("\u{3388}", "cal").corr(); // U+3388  ㎈  SQUARE CAL  cal
pub const SQUARE_KCAL: Correspondence<Linear> = Linear::new("\u{3389}", "kcal").corr(); // U+3389  ㎉  SQUARE KCAL     kcal
pub const SQUARE_PF: Correspondence<Linear> = Linear::new("\u{338A}", "pF").corr(); // U+338A  ㎊  SQUARE PF   p
pub const SQUARE_NF: Correspondence<Linear> = Linear::new("\u{338B}", "nF").corr(); // U+338B  ㎋  SQUARE NF   n
pub const SQUARE_MU_F: Correspondence<Linear> = Linear::new("\u{338C}", "µF").corr(); // U+338C  ㎌  SQUARE MU F     µF
pub const SQUARE_MU_G: Correspondence<Linear> = Linear::new("\u{338D}", "µg").corr(); // U+338D  ㎍  SQUARE MU G     µg
pub const SQUARE_MG: Correspondence<Linear> = Linear::new("\u{338E}", "mg").corr(); // U+338E  ㎎  SQUARE MG   mg
pub const SQUARE_KG: Correspondence<Linear> = Linear::new("\u{338F}", "kg").corr(); // U+338F  ㎏  SQUARE KG   kg
pub const SQUARE_HZ: Correspondence<Linear> = Linear::new("\u{3390}", "Hz").corr(); // U+3390  ㎐  SQUARE HZ   Hz
pub const SQUARE_KHZ: Correspondence<Linear> = Linear::new("\u{3391}", "kHz").corr(); // U+3391  ㎑  SQUARE KHZ  kHz
pub const SQUARE_MHZ: Correspondence<Linear> = Linear::new("\u{3392}", "MHz").corr(); // U+3392  ㎒  SQUARE MHZ  MHz
pub const SQUARE_GHZ: Correspondence<Linear> = Linear::new("\u{3393}", "GHz").corr(); // U+3393  ㎓  SQUARE GHZ  GHz
pub const SQUARE_THZ: Correspondence<Linear> = Linear::new("\u{3394}", "THz").corr(); // U+3394  ㎔  SQUARE THZ  THz
pub const SQUARE_MU_L: Correspondence<Linear> = Linear::new("\u{3395}", "µl").corr(); // U+3395  ㎕  SQUARE MU L     µl
pub const SQUARE_ML: Correspondence<Linear> = Linear::new("\u{3396}", "ml").corr(); // U+3396  ㎖  SQUARE ML   ml
pub const SQUARE_DL: Correspondence<Linear> = Linear::new("\u{3397}", "dl").corr(); // U+3397  ㎗  SQUARE DL   dl
pub const SQUARE_KL: Correspondence<Linear> = Linear::new("\u{3398}", "kl").corr(); // U+3398  ㎘  SQUARE KL   kl
pub const SQUARE_FM: Correspondence<Linear> = Linear::new("\u{3399}", "fm").corr(); // U+3399  ㎙  SQUARE FM   fm
pub const SQUARE_NM: Correspondence<Linear> = Linear::new("\u{339A}", "nm").corr(); // U+339A  ㎚  SQUARE NM   nm
pub const SQUARE_MU_M: Correspondence<Linear> = Linear::new("\u{339B}", "µm").corr(); // U+339B  ㎛  SQUARE MU M     µm
pub const SQUARE_MM: Correspondence<Linear> = Linear::new("\u{339C}", "mm").corr(); // U+339C  ㎜  SQUARE MM   mm
pub const SQUARE_CM: Correspondence<Linear> = Linear::new("\u{339D}", "cm").corr(); // U+339D  ㎝  SQUARE CM   cm
pub const SQUARE_KM: Correspondence<Linear> = Linear::new("\u{339E}", "km").corr(); // U+339E  ㎞  SQUARE KM   km
pub const SQUARE_MM_SQUARED: Correspondence<Linear> = Linear::new("\u{339F}", "mm").corr(); // U+339F  ㎟  SQUARE MM SQUARED   mm2     平方ミリメートル
pub const SQUARE_CM_SQUARED: Correspondence<Linear> = Linear::new("\u{33A0}", "cm2").corr(); // U+33A0  ㎠  SQUARE CM SQUARED   cm2     平方センチメートル
pub const SQUARE_M_SQUARED: Correspondence<Linear> = Linear::new("\u{33A1}", "m2").corr(); // U+33A1  ㎡  SQUARE M SQUARED    m2  平方メートル
pub const SQUARE_KM_SQUARED: Correspondence<Linear> = Linear::new("\u{33A2}", "km2").corr(); // U+33A2  ㎢  SQUARE KM SQUARED   km2     平方キロメートル
pub const SQUARE_MM_CUBED: Correspondence<Linear> = Linear::new("\u{33A3}", "mm3").corr(); // U+33A3  ㎣  SQUARE MM CUBED     mm3     立方ミリメートル
pub const SQUARE_CM_CUBED: Correspondence<Linear> = Linear::new("\u{33A3}", "cm3").corr(); // U+33A4  ㎤  SQUARE CM CUBED     cm3     立方センチメートル
pub const SQUARE_M_CUBED: Correspondence<Linear> = Linear::new("\u{33A5}", "m3").corr(); // U+33A5  ㎥  SQUARE M CUBED  m3  立方メートル
pub const SQUARE_KM_CUBED: Correspondence<Linear> = Linear::new("\u{33A6}", "km3").corr(); // U+33A6  ㎦  SQUARE KM CUBED     km3     立方キロメートル
pub const SQUARE_M_OVER_S: Correspondence<Linear> = Linear::new("\u{33A7}", "m/s").corr(); // U+33A7  ㎧  SQUARE M OVER S     m/s     メートル毎
pub const SQUARE_M_OVER_S_SQUARED: Correspondence<Linear> = Linear::new("\u{33A8}", "m/s2").corr(); // U+33A8  ㎨  SQUARE M OVER S SQUARED     m/s2    メートル毎秒毎秒
pub const SQUARE_PA: Correspondence<Linear> = Linear::new("\u{33A9}", "Pa").corr(); // U+33A9  ㎩  SQUARE PA   Pa
pub const SQUARE_KPA: Correspondence<Linear> = Linear::new("\u{33AA}", "kPa").corr(); // U+33AA  ㎪  SQUARE KPA  kPa
pub const SQUARE_MPA: Correspondence<Linear> = Linear::new("\u{33AB}", "MPa").corr(); // U+33AB  ㎫  SQUARE MPA  MPa
pub const SQUARE_GPA: Correspondence<Linear> = Linear::new("\u{33AC}", "GPa").corr(); // U+33AC  ㎬  SQUARE GPA  GPa
pub const SQUARE_RAD: Correspondence<Linear> = Linear::new("\u{33AD}", "rad").corr(); // U+33AD  ㎭  SQUARE RAD  rad
pub const SQUARE_RAD_OVER_S: Correspondence<Linear> = Linear::new("\u{33AE}", "rad/s").corr(); // U+33AE  ㎮  SQUARE RAD OVER S   rad/s   ラジアン毎
pub const SQUARE_RAD_OVER_S_SQUARED: Correspondence<Linear> =
    Linear::new("\u{33AF}", "rad/s2").corr(); // U+33AF  ㎯  SQUARE RAD OVER S SQUARED   rad/s2  ラジアン毎秒毎秒
pub const SQUARE_PS: Correspondence<Linear> = Linear::new("\u{33B0}", "ps").corr(); // U+33B0  ㎰  SQUARE PS   ps
pub const SQUARE_NS: Correspondence<Linear> = Linear::new("\u{33B1}", "ns").corr(); // U+33B1  ㎱  SQUARE NS   ns
pub const SQUARE_MU_S: Correspondence<Linear> = Linear::new("\u{33B2}", "µs").corr(); // U+33B2  ㎲  SQUARE MU S     µs
pub const SQUARE_MS: Correspondence<Linear> = Linear::new("\u{33B3}", "ms").corr(); // U+33B3  ㎳  SQUARE MS   ms
pub const SQUARE_PV: Correspondence<Linear> = Linear::new("\u{33B4}", "pV").corr(); // U+33B4  ㎴  SQUARE PV   p
pub const SQUARE_NV: Correspondence<Linear> = Linear::new("\u{33B5}", "nV").corr(); // U+33B5  ㎵  SQUARE NV   n
pub const SQUARE_MU_V: Correspondence<Linear> = Linear::new("\u{33B6}", "µV").corr(); // U+33B6  ㎶  SQUARE MU V     µV
pub const SQUARE_MV: Correspondence<Linear> = Linear::new("\u{33B7}", "mV").corr(); // U+33B7  ㎷  SQUARE MV   m
pub const SQUARE_KV: Correspondence<Linear> = Linear::new("\u{33B8}", "kV").corr(); // U+33B8  ㎸  SQUARE KV   k
pub const SQUARE_MV_MEGA: Correspondence<Linear> = Linear::new("\u{33B9}", "MV").corr(); // U+33B9  ㎹  SQUARE MV MEGA  MV
pub const SQUARE_PW: Correspondence<Linear> = Linear::new("\u{33BA}", "pW").corr(); // U+33BA  ㎺  SQUARE PW   p
pub const SQUARE_NW: Correspondence<Linear> = Linear::new("\u{33BB}", "nW").corr(); // U+33BB  ㎻  SQUARE NW   n
pub const SQUARE_MU_W: Correspondence<Linear> = Linear::new("\u{33BC}", "µW").corr(); // U+33BC  ㎼  SQUARE MU W     µW
pub const SQUARE_MW: Correspondence<Linear> = Linear::new("\u{33BD}", "mW").corr(); // U+33BD  ㎽  SQUARE MW   m
pub const SQUARE_KW: Correspondence<Linear> = Linear::new("\u{33BE}", "kW").corr(); // U+33BE  ㎾  SQUARE KW   k
pub const SQUARE_MW_MEGA: Correspondence<Linear> = Linear::new("\u{33BF}", "MW").corr(); // U+33BF  ㎿  SQUARE MW MEGA  MW
pub const SQUARE_K_OHM: Correspondence<Linear> = Linear::new("\u{33C0}", "kΩ").corr(); // U+33C0  ㏀  SQUARE K OHM    kΩ
pub const SQUARE_M_OHM: Correspondence<Linear> = Linear::new("\u{33C1}", "MΩ").corr(); // U+33C1  ㏁  SQUARE M OHM    MΩ
pub const SQUARE_AM: Correspondence<Linear> = Linear::new("\u{33C2}", "a.m.").corr(); // U+33C2  ㏂  SQUARE AM   a.m.
pub const SQUARE_BQ: Correspondence<Linear> = Linear::new("\u{33C3}", "Bq").corr(); // U+33C3  ㏃  SQUARE BQ   Bq
pub const SQUARE_CC: Correspondence<Linear> = Linear::new("\u{33C4}", "cc").corr(); // U+33C4  ㏄  SQUARE CC   cc  立方センチメートル
pub const SQUARE_CD: Correspondence<Linear> = Linear::new("\u{33C5}", "cd").corr(); // U+33C5  ㏅  SQUARE CD   cd  カンデラ
pub const SQUARE_C_OVER_KG: Correspondence<Linear> = Linear::new("\u{33C6}", "C/kg").corr(); // U+33C6  ㏆  SQUARE C OVER KG    C/kg    クーロン毎キログラム
pub const SQUARE_CO: Correspondence<Linear> = Linear::new("\u{33C7}", "Co.").corr(); // U+33C7  ㏇  SQUARE CO   Co.     Company
pub const SQUARE_DB: Correspondence<Linear> = Linear::new("\u{33C8}", "dB").corr(); // U+33C8  ㏈  SQUARE DB   d
pub const SQUARE_GY: Correspondence<Linear> = Linear::new("\u{33C9}", "Gy").corr(); // U+33C9  ㏉  SQUARE GY   Gy
pub const SQUARE_HA: Correspondence<Linear> = Linear::new("\u{33CA}", "ha").corr(); // U+33CA  ㏊  SQUARE HA   ha
pub const SQUARE_HP: Correspondence<Linear> = Linear::new("\u{33CB}", "HP").corr(); // U+33CB  ㏋  SQUARE HP   HP
pub const SQUARE_IN: Correspondence<Linear> = Linear::new("\u{33CC}", "in").corr(); // U+33CC  ㏌  SQUARE IN   in
pub const SQUARE_KK: Correspondence<Linear> = Linear::new("\u{33CD}", "K.").corr(); // U+33CD  ㏍  SQUARE KK   K.K.
pub const SQUARE_KM_CAPITAL: Correspondence<Linear> = Linear::new("\u{33CE}", "KM").corr(); // U+33CE  ㏎  SQUARE KM CAPITAL   KM
pub const SQUARE_KT: Correspondence<Linear> = Linear::new("\u{33CF}", "kt").corr(); // U+33CF  ㏏  SQUARE KT   kt
pub const SQUARE_LM: Correspondence<Linear> = Linear::new("\u{33D0}", "lm").corr(); // U+33D0  ㏐  SQUARE LM   lm
pub const SQUARE_LN: Correspondence<Linear> = Linear::new("\u{33D1}", "ln").corr(); // U+33D1  ㏑  SQUARE LN   ln
pub const SQUARE_LOG: Correspondence<Linear> = Linear::new("\u{33D2}", "log").corr(); // U+33D2  ㏒  SQUARE LOG  log
pub const SQUARE_LX: Correspondence<Linear> = Linear::new("\u{33D3}", "lx").corr(); // U+33D3  ㏓  SQUARE LX   lx
pub const SQUARE_MB_SMALL: Correspondence<Linear> = Linear::new("\u{33D4}", "mb").corr(); // U+33D4  ㏔  SQUARE MB SMALL     mb
pub const SQUARE_MIL: Correspondence<Linear> = Linear::new("\u{33D5}", "mil").corr(); // U+33D5  ㏕  SQUARE MIL  mil
pub const SQUARE_MOL: Correspondence<Linear> = Linear::new("\u{33D6}", "mol").corr(); // U+33D6  ㏖  SQUARE MOL  mol
pub const SQUARE_PH: Correspondence<Linear> = Linear::new("\u{33D7}", "pH").corr(); // U+33D7  ㏗  SQUARE PH   pH  水素イオン濃
pub const SQUARE_PM: Correspondence<Linear> = Linear::new("\u{33D8}", "p.m.").corr(); // U+33D8  ㏘  SQUARE PM   p.m.    午後
pub const SQUARE_PPM: Correspondence<Linear> = Linear::new("\u{33D9}", "ppm").corr(); // U+33D9  ㏙  SQUARE PPM  ppm
pub const SQUARE_PR: Correspondence<Linear> = Linear::new("\u{33DA}", "PR").corr(); // U+33DA  ㏚  SQUARE PR   PR
pub const SQUARE_SR: Correspondence<Linear> = Linear::new("\u{33DB}", "sr").corr(); // U+33DB  ㏛  SQUARE SR   sr
pub const SQUARE_SV: Correspondence<Linear> = Linear::new("\u{33DC}", "Sv").corr(); // U+33DC  ㏜  SQUARE SV   Sv
pub const SQUARE_WB: Correspondence<Linear> = Linear::new("\u{33DD}", "Wb").corr(); // U+33DD  ㏝  SQUARE WB   Wb
pub const SQUARE_V_OVER_M: Correspondence<Linear> = Linear::new("\u{33DE}", "V/m").corr(); // U+33DE  ㏞  SQUARE V OVER M     V/m     ボルト毎メート
pub const SQUARE_A_OVER_M: Correspondence<Linear> = Linear::new("\u{33DF}", "A/m").corr(); // U+33DF  ㏟  SQUARE A OVER M     A/m     アンペア毎メートル
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_ONE: Correspondence<Linear> =
    Linear::new("\u{33E0}", "1日").corr(); // U+33E0  ㏠  IDEOGRAPHIC TELEGRAPH SYMBOL FOR DAY ONE    1日     1日の漢字電信記
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_TWO: Correspondence<Linear> =
    Linear::new("\u{33E1}", "2日").corr(); // U+33E1  ㏡  IDEOGRAPHIC TELEGRAPH SYMBOL FOR DAY TWO    2日     2日の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_THREE: Correspondence<Linear> =
    Linear::new("\u{33E2}", "3日").corr(); // U+33E2  ㏢  IDEOGRAPHIC TELEGRAPH SYMBOL FOR DAY THREE  3日     3日の漢字電信記
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_FOUR: Correspondence<Linear> =
    Linear::new("\u{33E3}", "4日").corr(); // U+33E3  ㏣  IDEOGRAPHIC TELEGRAPH SYMBOL FOR DAY FOUR   4日     4日の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_FIVE: Correspondence<Linear> =
    Linear::new("\u{33E4}", "5日").corr(); // U+33E4  ㏤  IDEOGRAPHIC TELEGRAPH SYMBOL FOR DAY FIVE   5日     5日の漢字電信記
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_SIX: Correspondence<Linear> =
    Linear::new("\u{33E5}", "6日").corr(); // U+33E5  ㏥  IDEOGRAPHIC TELEGRAPH SYMBOL FOR DAY SIX    6日     6日の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_SEVEN: Correspondence<Linear> =
    Linear::new("\u{33E6}", "7日").corr(); // U+33E6  ㏦  IDEOGRAPHIC TELEGRAPH SYMBOL FOR DAY SEVEN  7日     7日の漢字電信記
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_EIGHT: Correspondence<Linear> =
    Linear::new("\u{33E7}", "8日").corr(); // U+33E7  ㏧  IDEOGRAPHIC TELEGRAPH SYMBOL FOR DAY EIGHT  8日     8日の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_NINE: Correspondence<Linear> =
    Linear::new("\u{33E8}", "9日").corr(); // U+33E8  ㏨  IDEOGRAPHIC TELEGRAPH SYMBOL FOR DAY NINE   9日     9日の漢字電信記
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_TEN: Correspondence<Linear> =
    Linear::new("\u{33E9}", "10日").corr(); // U+33E9  ㏩  IDEOGRAPHIC TELEGRAPH SYMBOL FOR DAY TEN    10日    10日の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_ELEVEN: Correspondence<Linear> =
    Linear::new("\u{33EA}", "11日").corr(); // U+33EA  ㏪  IDEOGRAPHIC TELEGRAPH SYMBOL FOR DAY ELEVEN     11日    11日の漢字電信記
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_TWELVE: Correspondence<Linear> =
    Linear::new("\u{33EB}", "12日").corr(); // U+33EB  ㏫  IDEOGRAPHIC TELEGRAPH SYMBOL FOR DAY TWELVE     12日    12日の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_THIRTEEN: Correspondence<Linear> =
    Linear::new("\u{33EC}", "13日").corr(); // U+33EC  ㏬  IDEOGRAPHIC TELEGRAPH SYMBOL FOR DAY THIRTEEN   13日    13日の漢字電信記
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_FOURTEEN: Correspondence<Linear> =
    Linear::new("\u{33ED}", "14日").corr(); // U+33ED  ㏭  IDEOGRAPHIC TELEGRAPH SYMBOL FOR DAY FOURTEEN   14日    14日の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_FIFTEEN: Correspondence<Linear> =
    Linear::new("\u{33EE}", "15日").corr(); // U+33EE  ㏮  IDEOGRAPHIC TELEGRAPH SYMBOL FOR DAY FIFTEEN    15日    15日の漢字電信記
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_SIXTEEN: Correspondence<Linear> =
    Linear::new("\u{33EF}", "16日").corr(); // U+33EF  ㏯  IDEOGRAPHIC TELEGRAPH SYMBOL FOR DAY SIXTEEN    16日    16日の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_SEVENTEEN: Correspondence<Linear> =
    Linear::new("\u{33F0}", "17日").corr(); // U+33F0  ㏰  IDEOGRAPHIC TELEGRAPH SYMBOL FOR DAY SEVENTEEN  17日    17日の漢字電信記
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_EIGHTEEN: Correspondence<Linear> =
    Linear::new("\u{33F1}", "18日").corr(); // U+33F1  ㏱  IDEOGRAPHIC TELEGRAPH SYMBOL FOR DAY EIGHTEEN   18日    18日の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_NINETEEN: Correspondence<Linear> =
    Linear::new("\u{33F2}", "19日").corr(); // U+33F2  ㏲  IDEOGRAPHIC TELEGRAPH SYMBOL FOR DAY NINETEEN   19日    19日の漢字電信記
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_TWENTY: Correspondence<Linear> =
    Linear::new("\u{33F3}", "20日").corr(); // U+33F3  ㏳  IDEOGRAPHIC TELEGRAPH SYMBOL FOR DAY TWENTY     20日    20日の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_TWENTY_ONE: Correspondence<Linear> =
    Linear::new("\u{33F4}", "21日").corr(); // U+33F4  ㏴  IDEOGRAPHIC TELEGRAPH SYMBOL FOR DAY TWENTY-ONE     21日    21日の漢字電信記
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_TWENTY_TWO: Correspondence<Linear> =
    Linear::new("\u{33F5}", "22日").corr(); // U+33F5  ㏵  IDEOGRAPHIC TELEGRAPH SYMBOL FOR DAY TWENTY-TWO     22日    22日の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_TWENTY_THREE: Correspondence<Linear> =
    Linear::new("\u{33F6}", "23日").corr(); // U+33F6  ㏶  IDEOGRAPHIC TELEGRAPH SYMBOL FOR DAY TWENTY-THREE   23日    23日の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_TWENTY_FOUR: Correspondence<Linear> =
    Linear::new("\u{33F7}", "24日").corr(); // U+33F7  ㏷  IDEOGRAPHIC TELEGRAPH SYMBOL FOR DAY TWENTY-FOUR    24日    24日の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_TWENTY_FIVE: Correspondence<Linear> =
    Linear::new("\u{33F8}", "25日").corr(); // U+33F8  ㏸  IDEOGRAPHIC TELEGRAPH SYMBOL FOR DAY TWENTY-FIVE    25日    25日の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_TWENTY_SIX: Correspondence<Linear> =
    Linear::new("\u{33F9}", "26日").corr(); // U+33F9  ㏹  IDEOGRAPHIC TELEGRAPH SYMBOL FOR DAY TWENTY-SIX     26日    26日の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_TWENTY_SEVEN: Correspondence<Linear> =
    Linear::new("\u{33FA}", "27日").corr(); // U+33FA  ㏺  IDEOGRAPHIC TELEGRAPH SYMBOL FOR DAY TWENTY-SEVEN   27日    27日の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_TWENTY_EIGHT: Correspondence<Linear> =
    Linear::new("\u{33FB}", "28日").corr(); // U+33FB  ㏻  IDEOGRAPHIC TELEGRAPH SYMBOL FOR DAY TWENTY-EIGHT   28日    28日の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_TWENTY_NINE: Correspondence<Linear> =
    Linear::new("\u{33FC}", "29日").corr(); // U+33FC  ㏼  IDEOGRAPHIC TELEGRAPH SYMBOL FOR DAY TWENTY-NINE    29日    29日の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_THIRTY: Correspondence<Linear> =
    Linear::new("\u{33FD}", "30日").corr(); // U+33FD  ㏽  IDEOGRAPHIC TELEGRAPH SYMBOL FOR DAY THIRTY     30日    30日の漢字電信記号
pub const IDEOGRAPHIC_TELEGRAPH_SYMBOL_FOR_DAY_THIRTY_ONE: Correspondence<Linear> =
    Linear::new("\u{33FE}", "31日").corr(); // U+33FE  ㏾  IDEOGRAPHIC TELEGRAPH SYMBOL FOR DAY THIRTY-ONE     31日    31日の漢字電信記号
pub const SQUARE_GAL: Correspondence<Linear> = Linear::new("\u{33FF}", "gal").corr(); // U+33FF  ㏿  SQUARE GAL  gal     ガル

pub const CJK_COMPATIBILITIES: Correspondence<CjkCompatibilities> =
    CjkCompatibilities::new().corr();
