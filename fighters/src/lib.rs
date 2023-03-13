pub fn install() {
    #[cfg(not(feature = "ignore-common"))]
    { common::install(); }
    #[cfg(feature = "include-pitb")]
    { pitb::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-eflame")]
    { eflame::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-pfushigisou")]
    { pfushigisou::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-purin")]
    { purin::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-younglink")]
    { younglink::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-plizardon")]
    { plizardon::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-shulk")]
    { shulk::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-chrom")]
    { chrom::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-miifighter")]
    { miifighter::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-gamewatch")]
    { gamewatch::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-krool")]
    { krool::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-samusd")]
    { samusd::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-ryu")]
    { ryu::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-rosetta")]
    { rosetta::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-yoshi")]
    { yoshi::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-lucas")]
    { lucas::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-link")]
    { link::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-lucario")]
    { lucario::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-zelda")]
    { zelda::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-kamui")]
    { kamui::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-wolf")]
    { wolf::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-nana")]
    { nana::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-tantan")]
    { tantan::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-koopag")]
    { koopag::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-ridley")]
    { ridley::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-gekkouga")]
    { gekkouga::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-ike")]
    { ike::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-roy")]
    { roy::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-rockman")]
    { rockman::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-peach")]
    { peach::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-elight")]
    { elight::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-gaogaen")]
    { gaogaen::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-falco")]
    { falco::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-ness")]
    { ness::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-kirby")]
    { kirby::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-pikachu")]
    { pikachu::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-daisy")]
    { daisy::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-sheik")]
    { sheik::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-mariod")]
    { mariod::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-luigi")]
    { luigi::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-captain")]
    { captain::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-mario")]
    { mario::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-jack")]
    { jack::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-palutena")]
    { palutena::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-dolly")]
    { dolly::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-pickel")]
    { pickel::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-wiifit")]
    { wiifit::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-pzenigame")]
    { pzenigame::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-pichu")]
    { pichu::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-pit")]
    { pit::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-ken")]
    { ken::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-brave")]
    { brave::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-richter")]
    { richter::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-miiswordsman")]
    { miiswordsman::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-robin")]
    { robin::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-master")]
    { master::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-robot")]
    { robot::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-szerosuit")]
    { szerosuit::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-ganon")]
    { ganon::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-wario")]
    { wario::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-diddy")]
    { diddy::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-shizue")]
    { shizue::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-littlemac")]
    { littlemac::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-pacman")]
    { pacman::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-edge")]
    { edge::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-marth")]
    { marth::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-trail")]
    { trail::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-mewtwo")]
    { mewtwo::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-metaknight")]
    { metaknight::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-toonlink")]
    { toonlink::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-koopajr")]
    { koopajr::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-sonic")]
    { sonic::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-donkey")]
    { donkey::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-inkling")]
    { inkling::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-packun")]
    { packun::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-miigunner")]
    { miigunner::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-buddy")]
    { buddy::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-pikmin")]
    { pikmin::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-cloud")]
    { cloud::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-fox")]
    { fox::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-duckhunt")]
    { duckhunt::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-lucina")]
    { lucina::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-murabito")]
    { murabito::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-bayonetta")]
    { bayonetta::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-samus")]
    { samus::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-koopa")]
    { koopa::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-simon")]
    { simon::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-popo")]
    { popo::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-dedede")]
    { dedede::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-snake")]
    { snake::install(cfg!(feature = "runtime")); }
    #[cfg(feature = "include-demon")]
    { demon::install(cfg!(feature = "runtime")); }
}

pub fn delayed_install() {
    #[cfg(feature = "include-elight")]
    { elight::delayed_install(); }
    #[cfg(feature = "include-metaknight")]
    { metaknight::delayed_install(); }
    #[cfg(feature = "include-ganon")]
    { ganon::delayed_install(); }
    #[cfg(feature = "include-littlemac")]
    { littlemac::delayed_install(); }
    #[cfg(feature = "include-wolf")]
    { wolf::delayed_install(); }
}