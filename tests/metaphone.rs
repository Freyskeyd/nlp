extern crate nlp;
use nlp::phonetics::metaphone::*;

#[test]
fn metaphone_1() {
    assert_eq!("EMN", metaphone("aeomon"))
}

#[test]
fn metaphone_2() {
    assert_eq!("NMN", metaphone("KNIMON"))
}

#[test]
fn metaphone_3() {
    assert_eq!("WLHK", metaphone("WHALLHACK"))
}

#[test]
fn metaphone_4() {
    assert_eq!("SN", metaphone("XENA"))
}

#[test]
fn metaphone_5() {
    assert_eq!("LXK", metaphone("LACIAC"))
}

fn assert_metaphone_equal(pairs: &[(&str, &str)]) {
    for i in pairs {
        let left = metaphone(i.0);
        let right = metaphone(i.1);
        assert!(left == right, "left: {}, right: {}, found: {:?}", left, right, i);
    }
}

fn assert_metaphone_to_many<T:ToString + ?Sized>(source: &str, matches: &[&T]) {

    for i in matches {
        let found = metaphone(*i);
        assert!(source == found);
    }
}

#[test]
fn metaphone_6() {
    assert_metaphone_equal(&vec![
                           ("Case", "case"),
                           ("CASE", "Case"),
                           ("caSe", "cAsE"),
                           ("quick", "cookie")
    ]);
}

#[test]
fn metaphone_7() {
    assert_metaphone_equal(&vec![
                           ("Lawrence", "Lorenza"),
                           ("Gary", "Cahra")
    ]);
}

#[test]
fn metaphone_8() {
    assert_eq!(metaphone("Aero"), metaphone("Eure"))
}

#[test]
fn metaphone_many_1() {
    assert_metaphone_to_many("WT", &vec![
                             "Wade", "Wait", "Waite",
                             "Wat", "Whit", "Wiatt",
                             "Wit", "Wittie", "Witty",
                             "Wood", "Woodie", "Woody"])
}

#[test]
fn metaphone_many_2() {
    assert_metaphone_to_many("ALBRT", &vec!["Ailbert", "Albert", "Alberto"]);
    assert_metaphone_to_many("ALBRK", &vec!["Alberik"]);
    assert_metaphone_to_many("ALBRXT", &vec!["Albrecht"])
}

#[test]
fn metaphone_many_3() {
    assert_metaphone_to_many("KR", &vec![
                             "Cahra", "Cara", "Carey",
                             "Cari", "Caria", "Carie",
                             "Caro", "Carree", "Carri",
                             "Carrie", "Carry", "Cary",
                             "Cora", "Corey", "Cori",
                             "Corie", "Correy", "Corri",
                             "Corrie", "Corry", "Cory",
                             "Gray", "Kara", "Kare",
                             "Karee", "Kari", "Karia",
                             "Karie", "Karrah", "Karrie",
                             "Karry", "Kary", "Keri",
                             "Kerri", "Kerrie", "Kerry",
                             "Kira", "Kiri", "Kora",
                             "Kore", "Kori", "Korie",
                             "Korrie", "Korry"])
}

#[test]
fn metaphone_many_4() {
    assert_metaphone_to_many("JN", &vec![
                             "Gena", "Gene", "Genia", "Genna", "Genni", "Gennie",
                             "Genny", "Giana", "Gianna", "Gina", "Ginni", "Ginnie",
                             "Ginny", "Jaine", "Jan", "Jana", "Jane", "Janey", "Jania",
                             "Janie", "Janna", "Jany", "Jayne", "Jean", "Jeana",
                             "Jeane",  "Jeanie", "Jeanna", "Jeanne", "Jeannie",
                             "Jen", "Jena", "Jeni", "Jenn", "Jenna", "Jennee", "Jenni",
                             "Jennie", "Jenny", "Jinny", "Jo Ann", "Jo-Ann", "Jo-Anne",
                             "Joan", "Joana", "Joane", "Joanie", "Joann", "Joanna", "Joanne",
                             "Joeann", "Johna", "Johnna", "Joni", "Jonie", "Juana",
                             "June", "Junia", "Junie"])
}

#[test]
fn metaphone_many_5() {
    assert_metaphone_to_many("NT", &vec![
                             "Hynda", "Nada", "Nadia", "Nady", "Nat", "Nata",
                             "Natty", "Neda", "Nedda", "Nedi", "Netta", "Netti",
                             "Nettie", "Netty", "Nita", "Nydia"])
}

#[test]
fn metaphone_many_6() {
    assert_metaphone_to_many("MR", &vec![
                             "Mair", "Maire", "Mara", "Mareah", "Mari", "Maria", "Marie",
                             "Mary", "Maura", "Maure", "Meara", "Merrie", "Merry", "Mira",
                             "Moira", "Mora", "Moria", "Moyra", "Muire", "Myra", "Myrah"])
}

#[test]
fn metaphone_many_7() {
    assert_metaphone_to_many("PRS", &vec![
                             "Pearcy", "Perris", "Piercy", "Pierz", "Pryse"])
}

#[test]
fn metaphone_many_8() {
    assert_metaphone_to_many("PTR", &vec![
                             "Peadar", "Peder", "Pedro", "Peter", "Petr",
                             "Peyter", "Pieter", "Pietro", "Piotr"])
}

#[test]
fn metaphone_many_9() {
    assert_metaphone_to_many("R", &vec![
                             "Ray", "Rey", "Roi", "Roy", "Ruy"])
}

#[test]
fn metaphone_many_10() {
    assert_metaphone_to_many("SSN", &vec![
                             "Siusan", "Sosanna", "Susan", "Susana", "Susann",
                             "Susanna", "Susannah", "Susanne", "Suzann", "Suzanna",
                             "Suzanne", "Zuzana"])
}

#[test]
fn metaphone_many_11() {
    assert_metaphone_to_many("RT", &vec![
                             "Rota", "Rudd", "Ryde"])
}

#[test]
fn metaphone_many_12() {
    assert_metaphone_to_many("SLN", &vec![
                             "Celene", "Celina", "Celine", "Selena", "Selene",
                             "Selina", "Seline", "Suellen", "Xylina"])
}

#[test]
fn metaphone_encode() {
    assert_eq!("HL", metaphone("howl"));
    assert_eq!("TSTNK", metaphone("testing"));
    assert_eq!("0", metaphone("The"));
    assert_eq!("KK", metaphone("quick"));
    assert_eq!("BRN", metaphone("brown"));
    assert_eq!("FKS", metaphone("fox"));
    assert_eq!("JMPT", metaphone("jumped"));
    assert_eq!("OFR", metaphone("over"));
    assert_eq!("0", metaphone("the"));
    assert_eq!("LS", metaphone("lazy"));
    assert_eq!("TKS", metaphone("dogs"));
    assert_eq!( "KM", metaphone("COMB") );
    assert_eq!( "TM", metaphone("TOMB") );
    assert_eq!( "WM", metaphone("WOMB") );
    assert_eq!( "SNS", metaphone("SCIENCE") );
    assert_eq!( "SN", metaphone("SCENE") );
    assert_eq!( "S", metaphone("SCY") );
    assert_eq!("", metaphone("WHY"));
    assert_eq!( "XP", metaphone("CIAPO") );
    assert_eq!( "SKTL", metaphone("SCHEDULE") );
    assert_eq!( "SKMTK", metaphone("SCHEMATIC") );
    assert_eq!( "KRKTR", metaphone("CHARACTER") );
    assert_eq!( "TX", metaphone("TEACH") );
    assert_eq!( "TJ", metaphone("DODGY") );
    assert_eq!( "TJ", metaphone("DODGE") );
    assert_eq!( "AJMT", metaphone("ADGIEMTI") );
    assert_eq!( "KNT", metaphone("GHENT") );
    assert_eq!( "B", metaphone("BAUGH") );
    // NOTE: This does not test for silent GN, but for starting with GN
    assert_eq!( "N", metaphone("GNU") );
    // NOTE: Trying to test for GNED, but expected code does not appear to execute
    assert_eq!( "SNT", metaphone("SIGNED") );
    assert_eq!( "FX", metaphone("PHISH") );
    assert_eq!( "XT", metaphone("SHOT") );
    assert_eq!( "OTXN", metaphone("ODSIAN") );
    assert_eq!( "PLXN", metaphone("PULSION") );
    assert_eq!( "OX", metaphone("OTIA") );
    assert_eq!( "PRXN", metaphone("PORTION") );
    assert_eq!( "RX", metaphone("RETCH") );
    assert_eq!( "WX", metaphone("WATCH") );
    // should be AKSKS, but istruncated by Max Code Length
    assert_eq!( "AKSKS", metaphone("AXEAXE") );
}

#[test]
fn metaphone_9() {
    let aero: String = "Aero".to_owned();
    let eure: String = "Eure".to_owned();

    assert_eq!(metaphone(&aero), metaphone(&eure))
}

#[test]
fn metaphone_many_13() {

    let v: Vec<String> = vec![
        "Wade".to_owned(), "Wait".to_owned(), "Waite".to_owned(),
        "Wat".to_owned(), "Whit".to_owned(), "Wiatt".to_owned(),
        "Wit".to_owned(), "Wittie".to_owned(), "Witty".to_owned(),
        "Wood".to_owned(), "Woodie".to_owned(), "Woody".to_owned()];

    let wt: String = "WT".to_owned();

    let mut vec: Vec<&String> = Vec::new();
    for vv in &v {
        vec.push(&vv);
    }
    assert_metaphone_to_many(&wt, &vec[..])
}

#[test]
fn metaphone_11() {
    assert_eq!(metaphone("dacite"), "TST")
}

#[test]
fn metaphone_12() {
    assert_eq!(metaphone("éréction"), "TST")
}
