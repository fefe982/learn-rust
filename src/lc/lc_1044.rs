// https://leetcode.com/problems/longest-duplicate-substring/
// 1044. Longest Duplicate Substring
pub struct Solution;
impl Solution {
    pub fn longest_dup_substring(s: String) -> String {
        let pr = 1_0000_0000_7;
        let ss = s;
        let s = ss.as_bytes();
        let n = s.len();
        if s[0..n - 1] == s[1..] {
            return ss[1..].to_owned();
        }
        let mut pow = vec![];
        let mut ppr = 1;
        for _ in 0..n {
            pow.push(ppr);
            ppr = (ppr * 26) % pr;
        }
        let mut high = n - 1;
        let mut low = 1;
        let mut m = std::collections::HashMap::<i64, Vec<usize>>::new();
        let mut lowi = n;
        for i in 0..n {
            let v = m.entry(s[i] as i64).or_default();
            if !v.is_empty() {
                lowi = v[0];
                break;
            }
            v.push(i + 1);
        }
        if lowi == n {
            return "".to_string();
        }
        while low + 1 < high {
            m.clear();
            let mid = (low + high) / 2;
            let mut h = 0;
            for i in 0..mid {
                h = (h * 26 + (s[i] - b'a') as i64) % pr;
            }
            m.insert(h, vec![mid]);
            let mut midi = n;
            'f: for i in mid..n {
                h = (h * 26 + (s[i] - b'a') as i64 + pr - (pow[mid] * ((s[i - mid] - b'a') as i64) % pr)) % pr;
                let v = m.entry(h).or_default();
                for vi in v.iter() {
                    if s[*vi - mid..*vi] == s[i - mid + 1..i + 1] {
                        midi = *vi;
                        break 'f;
                    }
                }
                v.push(i + 1);
            }
            if midi == n {
                high = mid;
            } else {
                low = mid;
                lowi = midi;
            }
        }
        ss[lowi - low..lowi].to_owned()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_longest_dup_substring() {
        assert_eq!(Solution::longest_dup_substring("vbaemlrfvasubbuxdqohlpkuuznigzebcegorztmbngdocautqbegnbamqitrowtucjldexfsgiiiicbyeigrjjgbtbsiznccwohanmutudceffflnlfywnbqotypictesbhgndbkgfooltzahgbdtctjytzgwsnotwwhzyrifhwqbxtkewnxyjhycgfaauruqpbrnuztbxlevgobccydnhbppisqelmuapoqjnlmnyrkfflhjlkwvookwgcbtoxtdnlobwbqvsaazljjywftktgibiluzlqeybtbawrxqtzpeiyfggysbebdpozozpuatpbvlvsbortvbtaizaabfbgushkrxgtswvnhqflgyebzzdhzeicllkhdrrxqxqivnxdfeqmcupctvluztykhirmcklsjcdfrckxhallwcprxboywmidxbqbnbtzfygzbqyksjvqnljloxjkmuolxgmljhmrgsqjkovezqwmdfxrfjctfsecaspjzbahvqsfgnlfghsjnrduchnhzrkzlbkkdnuyfogmcrrkzltofyihfvpgobhzcslffhgkwowafqkgdduieeqjbhqpixqkrcswlajhohhvxrcdxylbfdmgkeyueohvsbuebdlrkvmkmonlpaougbashynlpujhvdnyizpuwdgiuvwjwkfenfwvgjixfrgcwuaevrbrvhbpdxabkffxmduundrswyxtszznrceomffpfhspmzoycltrqmmmkjhcijrblvrxdyokfibrjrsmcndszrhkaqolzgmndgkiebuklvdvnvhrkmmkgvuglkrlqgaummesnzczwiujfddqrraxmwgxgzkxfkttxzmkyqlvyhggmlwecwdoujoajyzqjukiuyuuogrppavcntiqrnglzunkvnagjmofmicbqtsgvpkjiubzhhmiezszovfahvfudjxufrcaiolqimrrufaaguzvpgxxbetgosctlgvtuhletcwpjyppsqmiyjhbocicukxhrsanzlsvpqkyocniucqtlzubbefxpceixexwpprgheevgueawwvhwqjzuqxltdvqitozkigsmmpugvwcssmqkxpgnouispyckpbvxjplwehcadhuxtmjqbsdqmmkdmmqdpnazcnxiubkaiezbqlrudxmrjwwximbzzbydrxmlzwnhdnjtycpdzbukxsihfjjgmhmdfyrgwgnztyvtdkwcmspllxvsgeaunmaumrajzqivofotqnwehlrskomsunmkzmemfhxfxfannqhfdfllsefctsuphgzgvguanrfmzhzmyitdneqfiwjdqdtzmufhachntpzuwbgwjbazfqgqoqtgtofhjnojpwzknafldnippkowcwmtmiziwhphuetlcsxruecyumnlwdhbzwuoevwccnkytdlxxjorptrjfeuehcfhabvewtizguzczlzwwqqljhxqmruvvbcvczbbsmcbgbfgoeemwuyfexwqosnmwqjyrnbmzdpffunvasixmieqfcivvwhrotrhgvtfyimfcabfsjcgtgxyjhvgffztysbenwlmhxtjxuopfswybvykeuskztdzlgdbfonmngxldocigajhxhtcdnmegrdsagvfwirkqqxvebelcpubwqdcdpgmjkgjsbqpjjbbzaxebdrfsnhqdksjcbvlnbwfcmwnxxqwxpcbstgmjndmaxxcaspnvntzzpmypfuouxrpsybblgqlgnpgiewdtpdsgrsxynovmfkefnirwyecnukvaibuzeoixhfywvtiddmiqwbmugqicddcleylkptkwaiswbcbgqjynlnhtfrvixzwdyhonaalckfalhbtxnrkagutmlowipsmnzccbepeouebdiubvnnvwjqiveefvxtlyxnabfgpzmqdwmgrcvoyfegckmhgldrkagmelutnimsfyyyctnaaticgvkmipfuplgtzcpmlbeqbeoezdgaxpcdmmrudwrtjxoicdwngadketqexwewnizbxsyaqptzokkqpgdwhginaytzwzvmiwnfolfeabfplpfawlydristimsflprtleazsgfshljucrwycvdvfwrokltvbmrmsrokzvfgiqalhzjlwnhblapvlksmyjbrdpdcmklvbngkmrwiefkrfbmqoffmqgfvxlyjvjgyymllghtkofevjqnxxslqarlzejlzuratglemuzyihcfqvspslaintunxfyzapzvxmbjgaiqjmvbhpqactkqmgdmmdloumlsamdionyyavzhksjnlwdzcilqnyghmrzmdfussussrrccjlrwjwwoehetilauecjkhfczcuutnwvzjmvtirhobnhjakffmqpvwtwypqrcmpdwdzrosycvlmgoymtmckhaddaswswijagmbxatxfpltvzrjudoemohppznjxwsxegaehudmlofipyujbinnywepxsavqsuwzvaireweudombixeslrtjxihrjehdyeetgommkqjpfvqpsuifnbrnlyewcoccuiycxkpjbyzivrxpohvbkmqwmxuqlwshfqjedyngymotenjsvgvfuodilyyywqfbjofmpvsnhijnvufscdayrdgcdeaawhdhxltmgncnvndtjkllnllerriuxkwvtadrnydtmhijulcdwoolbzsssaprnlngnsxaqdvekeqfrsinizzkarevttmihzpwrozkruqcagjszthjsitvbaqxtcfcewzxlajeuaixgrrhlarfjmammsjivrkbnlyalihikphqjyywyrbjdewqwhytykuowvqjfxtppahaahwzyzkcdgscqsvocseapxmuwfllisniajjajucocwqsoojtgnvvmhrjpcwxwkhyvrzgnbvpaniqbtjjihevqxefbaeymoyihxoubljbztrcodmxmsscqwktyqrnesvlplhsxtvyopadumaghskxquqqnkdreafvmmolhjjwylzuazahtlgjhybukfoohvktjygrjpnkbmednhkgsqrbosyzzopjjzcszjllaaxdobgbkqjeiagzvgawupfdrkxqvjdvlabaehboaltknbjihjhbbmgswdkroopaoqauzdjboeoehxoojvpmkvegaoaperrvjrpkwamlgcrnzfbkbswlpctstvjxmauxtkipmwctytizzmkcgipwnkqgizfwzausmcjaymvwheviqhkwsbufoknwlssksmegqohqccojgmsytexnirswehspamcubibryiiamwmvketekqtskooqvihvvjswutpsdztryfmilgmvyiokwcarnkpivhgpcvihvdytklbvrqsrygcwlenyqeimlodpjnweqvzagbxztqsrywnlbcjthtfuhjktwgxpdzrjarkvjdmhzbtcfewmydjhmtalbacfztqbmbwvgkdhlykseghejqmylmlmmptnkkoxnmstwxajvmccujggbsgmhtaviujsgzptmiokfqudfyfzjnfkgnwpgwkosvjyvytjihtqaptfnaewnmivzekgdxwxsfwnepyozsebhzgxzmtnackaivinselmmjcsyagzslackgzmvubvylxhifwcmhpyqrqrrwsmjtqwbriwljukurtdzrhlzwjrpbkjzdvjwnfzenyrysdslwxkofvpdgitjtcjwzsumztsotjmlpexnejdlfsondjxwlgumaoabjspdebqvhpilzmygenvjysfkazycpzdbtsgztwubmdonpnvvcqfajmmlolodmrgfmmhfjbixgxmxkhtmxamiobucmkiydofpdmvoqdnwczumipyfpnqfokicwsczihplwmzmrmiunegvthdkwtqmfaobsqcczroxfpjxeuttybpskucgnussuivoaxrarhzwwlvcfqwfslxcfcluydotmljmiaobhoagzxigoelirlmypxsndiiptsdcuufekfkldurtbcbupntennsqwchrukvrgbvqghbldseexhzovutijnahphzmrudyvyqefspimwpldqaaktkvikdxrcjlymlwkbartrcinrttjttvoeqwovuixftyseuehuydnaldtewcarxovplmlhukmpietclayjfmjqpexnixehlcwyojfkuszeyuhlwfuctfuijrvtocacexauiigcafhabqmzxidroybzpxtgztifskzbsffumovpejoeruvhswhjvcpwxzbskcvcjazvrwmxhdbyxwirvraqtsnqrqidtdieghaxfowmrykowufrlikeazxtphzknhpvqruvfqhxotnlovczdogfnfadozpbggwwxstfjaexutkiopjtdrarwbwlalojunhtfbyobeoxyyhmnririikfjgsmtgvntehbfhpjmdgmeoyrhikpruwcaqqwxnjssstzcliqqrcufcoolydcvcgxsxtrkfkexfezqmrjdtdkwryhselnitmaqgsdlehkjnkccblhxqutksacynrggdjxldcwhlhsbtwdwhktyemizomzbfikkrjwuludydxzwucvbpobtdlutzuvgcfrvqprblubptblnfgruxuqagmvhgqokxhhnyyqjuuyovmbcsuxrpptxbpekhuwhdewbcplzjjpevsiqnfjcwdzaufkbcgifkfjpuuqfffjdxrvmzeoxjpxdxhfzqpgcwptqljvrqgwoarabvrahiykfhpgxhpcdevwshtlxchlcyofvffcnfpvngbmnsqrzmnrtgcqkjemjstezzjmgyjtgniufynemavfizytichtubavcjhijtepgollmyqzangjneexgnrcqrxuchfncjcizqtlolmpbtaozenitemkxmebvjoxutftjyhxmtmnoodqsqyoxkywycytomqifvcowjokeaxvpaljsjxpvxoucmpqygaebcuznijulipckandlnugkicousevafbdjvdzgottxivhikmtxjbkrgyoyfjykmbvhgupvniuxfdfqqapzczsiagifrirdlmnsjepwnjwsmbkeeadizysbgagaixftmsnvxctmlpeatrexrkfsixuqzaqhawcmqshpuabiqiijzmisynkikezjhrshpwvgocxleztujbfkpncocmwotxzaptjeqemgikrmlkjulqedggriupxpnwrcqaiigxoqhidssoogeujmcavwrumesdiigrsoojfxrirlyhrebardcmbugnaytjjzcdnsmupfeircpihslavpmwlummuhxfgzjvtxskdwsuzjeibriyhwwusiimpfsxgdvzcnjflbbkmgunxengakbssjhkrbjxeexitgofkrrwxomxszfvjgnesuqjxzgbgdkzmagwraurogqiivdterxwnphlpnovtawhcffufznviddqyhjcajanyhapxpksrjzuoeqvfdqngvfgcpjeajuatusqlckdnjzdppyuuqiamngqervgdpjwlboxfyctjdwysxuopcobswuyqrhtgqrtrhswhurqzqqtpkhepvgjuchktofbgyuxfnwmjpejhuewgmbxjjgdlqpgguhnsdmzsklccrnjnriufjrpuashsknyeunzzcysokwsdjercmlixezrgtyydnzijohisfajrwdxhhomvzwsfvlmolmsylchclqwssfkpjjyqxkmyigdfsrudeoerqlvbdstwxfnionbnanlivpopiktmazgqkbtqtootwbbmqcaqrlzncpclbxzbwhtjlmbecpzysbnidnzxaamdrqqsorvmboxmcasepsfjtssyhtxhvjmoaegqydezrcntcfzuxedsyxrfsoppaddailqioujfywadbazzgethekowbdmdjrdtdvbrrkzsgzvhbwiiacreofdrlruuiznluofmyeggfdphzyrbciisplkaceukehyaxdovjudoxtxwtqavyinqtzqxglhksfmqkbntvtkvmhtfytcybrowrhtzsdjmixevysowlarzikgigkihbzgugztacemncriclyywzrxjcdtkndrqczlkzgkdnxqpqatbzuzalwlpzezohtemsrytylxlkpcaqxbrrardycsxiunrnrffjebjpywznabdxcwpenucrobbiotvhadseebvezwrzxxzztqfjkhcykgjabgibjagpedvdanfxmexhuesemxeydnzeuhffjnxntstthyqvcpdwuxciuigxyfdsolzyayntrgmaefwiubeqyiytrhspmwjdjkdjjqdcxrcdwamrbshabivupnldlwguglercjvbaaexasoclxeofzkumxyytgubsyvwhqxiqdhtwvvjszzaalumiumbdjevhzrrsqbktidrfaczbdzbowqwsezvngsxflozbrkxbpqgqvhryhtnfjlzplvdrpaybqyejkbkzusrpzjnieapnmpczkhqzhqczqhiciscckvrmehuijkxvzcwljomtfpsvwbygtclgxeselomvamsbormxfbqksliqmiwhmjplojpbyeyyqtcekqdprwcjhmzvuycihstxbjbbnbcduejgumwmkaxrmzmgzroijhgmsjohksmhwvnqkleulemxdafcrtkcdsrxdffqzrxvnmnzyutjyhhdimhbitenovkrcrjbjgyyvnxthsehalkkatlknrxdmjwqbgtmmtkhhjcobhwduinuczgrqdufpqxqwtclmoutzzkcgigaqtxuudlnjhsyarqykulxkgsjfclnmtpdnozjslwduqduitvupgrzmmitqvidpyiemhngumlpcolmjghynaxbmfxfdfisfsyuzncuzojccwqmdxkqyitmpqrsybmftkzvycpzqaduwugbttttbngsfznddzjymktmmklekpzjlfkeyeybtgwyhjcmknlwxgkmryqdppmavxevdezwmvuueygqntplazxtxnwmfjocivxlzyhotdizeqqrkfcmgbferbswkhysexffwotrsbrwuhneossuvxamavklekfiknnibhztkqrezfipzckuzmjahvnliuvshjclsbecuyhtdrleuvatjjvrhkepfajollzdmgfgemcjeppampvvzrmibtxivgxgtyjfeookdsvjhkjtaeobvdjzyghtogzhfiolyewbyrkfcvaearfxwowuwgnmmovrwldwszyqskwwgyaiphflxehvkwjwkeqistfkufaorylxxnhovncutjqdgzbsgrbamimgnmxeniemxlauaepvqhyyicqottqibcqqrnxevdqvqsprzgopnnnwrdmmxfuahlryyoewtwrjricqprfcguaxzpjwuezbpqcpgglzdckunnkcereklhhkwsjqwirnavficqjfvtziglkkeqwrzfdvymnwwhmycrgejrjelkorxaebtcssivbaemlrfvasubbuxdqohlpkuuznigzebcegorztmbngdocautqbegnbamqitrowtucjldexfsgiiiicbyeigrjjgbtbsiznccwohanmutudceffflnlfywnbqotypictesbhgndbkgfooltzahgbdtctjytzgwsnotwwhzyrifhwqbxtkewnxyjhycgfaauruqpbrnuztbxlevgobccydnhbppisqelmuapoqjnlmnyrkfflhjlkwvookwgcbtoxtdnlobwbqvsaazljjywftktgibiluzlqeybtbawrxqtzpeiyfggysbebdpozozpuatpbvlvsbortvbtaizaabfbgushkrxgtswvnhqflgyebzzdhzeicllkhdrrxqxqivnxdfeqmcupctvluztykhirmcklsjcdfrckxhallwcprxboywmidxbqbnbtzfygzbqyksjvqnljloxjkmuolxgmljhmrgsqjkovezqwmdfxrfjctfsecaspjzbahvqsfgnlfghsjnrduchnhzrkzlbkkdnuyfogmcrrkzltofyihfvpgobhzcslffhgkwowafqkgdduieeqjbhqpixqkrcswlajhohhvxrcdxylbfdmgkeyueohvsbuebdlrkvmkmonlpaougbashynlpujhvdnyizpuwdgiuvwjwkfenfwvgjixfrgcwuaevrbrvhbpdxabkffxmduundrswyxtszznrceomffpfhspmzoycltrqmmmkjhcijrblvrxdyokfibrjrsmcndszrhkaqolzgmndgkiebuklvdvnvhrkmmkgvuglkrlqgaummesnzczwiujfddqrraxmwgxgzkxfkttxzmkyqlvyhggmlwecwdoujoajyzqjukiuyuuogrppavcntiqrnglzunkvnagjmofmicbqtsgvpkjiubzhhmiezszovfahvfudjxufrcaiolqimrrufaaguzvpgxxbetgosctlgvtuhletcwpjyppsqmiyjhbocicukxhrsanzlsvpqkyocniucqtlzubbefxpceixexwpprgheevgueawwvhwqjzuqxltdvqitozkigsmmpugvwcssmqkxpgnouispyckpbvxjplwehcadhuxtmjqbsdqmmkdmmqdpnazcnxiubkaiezbqlrudxmrjwwximbzzbydrxmlzwnhdnjtycpdzbukxsihfjjgmhmdfyrgwgnztyvtdkwcmspllxvsgeaunmaumrajzqivofotqnwehlrskomsunmkzmemfhxfxfannqhfdfllsefctsuphgzgvguanrfmzhzmyitdneqfiwjdqdtzmufhachntpzuwbgwjbazfqgqoqtgtofhjnojpwzknafldnippkowcwmtmiziwhphuetlcsxruecyumnlwdhbzwuoevwccnkytdlxxjorptrjfeuehcfhabvewtizguzczlzwwqqljhxqmruvvbcvczbbsmcbgbfgoeemwuyfexwqosnmwqjyrnbmzdpffunvasixmieqfcivvwhrotrhgvtfyimfcabfsjcgtgxyjhvgffztysbenwlmhxtjxuopfswybvykeuskztdzlgdbfonmngxldocigajhxhtcdnmegrdsagvfwirkqqxvebelcpubwqdcdpgmjkgjsbqpjjbbzaxebdrfsnhqdksjcbvlnbwfcmwnxxqwxpcbstgmjndmaxxcaspnvntzzpmypfuouxrpsybblgqlgnpgiewdtpdsgrsxynovmfkefnirwyecnukvaibuzeoixhfywvtiddmiqwbmugqicddcleylkptkwaiswbcbgqjynlnhtfrvixzwdyhonaalckfalhbtxnrkagutmlowipsmnzccbepeouebdiubvnnvwjqiveefvxtlyxnabfgpzmqdwmgrcvoyfegckmhgldrkagmelutnimsfyyyctnaaticgvkmipfuplgtzcpmlbeqbeoezdgaxpcdmmrudwrtjxoicdwngadketqexwewnizbxsyaqptzokkqpgdwhginaytzwzvmiwnfolfeabfplpfawlydristimsflprtleazsgfshljucrwycvdvfwrokltvbmrmsrokzvfgiqalhzjlwnhblapvlksmyjbrdpdcmklvbngkmrwiefkrfbmqoffmqgfvxlyjvjgyymllghtkofevjqnxxslqarlzejlzuratglemuzyihcfqvspslaintunxfyzapzvxmbjgaiqjmvbhpqactkqmgdmmdloumlsamdionyyavzhksjnlwdzcilqnyghmrzmdfussussrrccjlrwjwwoehetilauecjkhfczcuutnwvzjmvtirhobnhjakffmqpvwtwypqrcmpdwdzrosycvlmgoymtmckhaddaswswijagmbxatxfpltvzrjudoemohppznjxwsxegaehudmlofipyujbinnywepxsavqsuwzvaireweudombixeslrtjxihrjehdyeetgommkqjpfvqpsuifnbrnlyewcoccuiycxkpjbyzivrxpohvbkmqwmxuqlwshfqjedyngymotenjsvgvfuodilyyywqfbjofmpvsnhijnvufscdayrdgcdeaawhdhxltmgncnvndtjkllnllerriuxkwvtadrnydtmhijulcdwoolbzsssaprnlngnsxaqdvekeqfrsinizzkarevttmihzpwrozkruqcagjszthjsitvbaqxtcfcewzxlajeuaixgrrhlarfjmammsjivrkbnlyalihikphqjyywyrbjdewqwhytykuowvqjfxtppahaahwzyzkcdgscqsvocseapxmuwfllisniajjajucocwqsoojtgnvvmhrjpcwxwkhyvrzgnbvpaniqbtjjihevqxefbaeymoyihxoubljbztrcodmxmsscqwktyqrnesvlplhsxtvyopadumaghskxquqqnkdreafvmmolhjjwylzuazahtlgjhybukfoohvktjygrjpnkbmednhkgsqrbosyzzopjjzcszjllaaxdobgbkqjeiagzvgawupfdrkxqvjdvlabaehboaltknbjihjhbbmgswdkroopaoqauzdjboeoehxoojvpmkvegaoaperrvjrpkwamlgcrnzfbkbswlpctstvjxmauxtkipmwctytizzmkcgipwnkqgizfwzausmcjaymvwheviqhkwsbufoknwlssksmegqohqccojgmsytexnirswehspamcubibryiiamwmvketekqtskooqvihvvjswutpsdztryfmilgmvyiokwcarnkpivhgpcvihvdytklbvrqsrygcwlenyqeimlodpjnweqvzagbxztqsrywnlbcjthtfuhjktwgxpdzrjarkvjdmhzbtcfewmydjhmtalbacfztqbmbwvgkdhlykseghejqmylmlmmptnkkoxnmstwxajvmccujggbsgmhtaviujsgzptmiokfqudfyfzjnfkgnwpgwkosvjyvytjihtqaptfnaewnmivzekgdxwxsfwnepyozsebhzgxzmtnackaivinselmmjcsyagzslackgzmvubvylxhifwcmhpyqrqrrwsmjtqwbriwljukurtdzrhlzwjrpbkjzdvjwnfzenyrysdslwxkofvpdgitjtcjwzsumztsotjmlpexnejdlfsondjxwlgumaoabjspdebqvhpilzmygenvjysfkazycpzdbtsgztwubmdonpnvvcqfajmmlolodmrgfmmhfjbixgxmxkhtmxamiobucmkiydofpdmvoqdnwczumipyfpnqfokicwsczihplwmzmrmiunegvthdkwtqmfaobsqcczroxfpjxeuttybpskucgnussuivoaxrarhzwwlvcfqwfslxcfcluydotmljmiaobhoagzxigoelirlmypxsndiiptsdcuufekfkldurtbcbupntennsqwchrukvrgbvqghbldseexhzovutijnahphzmrudyvyqefspimwpldqaaktkvikdxrcjlymlwkbartrcinrttjttvoeqwovuixftyseuehuydnaldtewcarxovplmlhukmpietclayjfmjqpexnixehlcwyojfkuszeyuhlwfuctfuijrvtocacexauiigcafhabqmzxidroybzpxtgztifskzbsffumovpejoeruvhswhjvcpwxzbskcvcjazvrwmxhdbyxwirvraqtsnqrqidtdieghaxfowmrykowufrlikeazxtphzknhpvqruvfqhxotnlovczdogfnfadozpbggwwxstfjaexutkiopjtdrarwbwlalojunhtfbyobeoxyyhmnririikfjgsmtgvntehbfhpjmdgmeoyrhikpruwcaqqwxnjssstzcliqqrcufcoolydcvcgxsxtrkfkexfezqmrjdtdkwryhselnitmaqgsdlehkjnkccblhxqutksacynrggdjxldcwhlhsbtwdwhktyemizomzbfikkrjwuludydxzwucvbpobtdlutzuvgcfrvqprblubptblnfgruxuqagmvhgqokxhhnyyqjuuyovmbcsuxrpptxbpekhuwhdewbcplzjjpevsiqnfjcwdzaufkbcgifkfjpuuqfffjdxrvmzeoxjpxdxhfzqpgcwptqljvrqgwoarabvrahiykfhpgxhpcdevwshtlxchlcyofvffcnfpvngbmnsqrzmnrtgcqkjemjstezzjmgyjtgniufynemavfizytichtubavcjhijtepgollmyqzangjneexgnrcqrxuchfncjcizqtlolmpbtaozenitemkxmebvjoxutftjyhxmtmnoodqsqyoxkywycytomqifvcowjokeaxvpaljsjxpvxoucmpqygaebcuznijulipckandlnugkicousevafbdjvdzgottxivhikmtxjbkrgyoyfjykmbvhgupvniuxfdfqqapzczsiagifrirdlmnsjepwnjwsmbkeeadizysbgagaixftmsnvxctmlpeatrexrkfsixuqzaqhawcmqshpuabiqiijzmisynkikezjhrshpwvgocxleztujbfkpncocmwotxzaptjeqemgikrmlkjulqedggriupxpnwrcqaiigxoqhidssoogeujmcavwrumesdiigrsoojfxrirlyhrebardcmbugnaytjjzcdnsmupfeircpihslavpmwlummuhxfgzjvtxskdwsuzjeibriyhwwusiimpfsxgdvzcnjflbbkmgunxengakbssjhkrbjxeexitgofkrrwxomxszfvjgnesuqjxzgbgdkzmagwraurogqiivdterxwnphlpnovtawhcffufznviddqyhjcajanyhapxpksrjzuoeqvfdqngvfgcpjeajuatusqlckdnjzdppyuuqiamngqervgdpjwlboxfyctjdwysxuopcobswuyqrhtgqrtrhswhurqzqqtpkhepvgjuchktofbgyuxfnwmjpejhuewgmbxjjgdlqpgguhnsdmzsklccrnjnriufjrpuashsknyeunzzcysokwsdjercmlixezrgtyydnzijohisfajrwdxhhomvzwsfvlmolmsylchclqwssfkpjjyqxkmyigdfsrudeoerqlvbdstwxfnionbnanlivpopiktmazgqkbtqtootwbbmqcaqrlzncpclbxzbwhtjlmbecpzysbnidnzxaamdrqqsorvmboxmcasepsfjtssyhtxhvjmoaegqydezrcntcfzuxedsyxrfsoppaddailqioujfywadbazzgethekowbdmdjrdtdvbrrkzsgzvhbwiiacreofdrlruuiznluofmyeggfdphzyrbciisplkaceukehyaxdovjudoxtxwtqavyinqtzqxglhksfmqkbntvtkvmhtfytcybrowrhtzsdjmixevysowlarzikgigkihbzgugztacemncriclyywzrxjcdtkndrqczlkzgkdnxqpqatbzuzalwlpzezohtemsrytylxlkpcaqxbrrardycsxiunrnrffjebjpywznabdxcwpenucrobbiotvhadseebvezwrzxxzztqfjkhcykgjabgibjagpedvdanfxmexhuesemxeydnzeuhffjnxntstthyqvcpdwuxciuigxyfdsolzyayntrgmaefwiubeqyiytrhspmwjdjkdjjqdcxrcdwamrbshabivupnldlwguglercjvbaaexasoclxeofzkumxyytgubsyvwhqxiqdhtwvvjszzaalumiumbdjevhzrrsqbktidrfaczbdzbowqwsezvngsxflozbrkxbpqgqvhryhtnfjlzplvdrpaybqyejkbkzusrpzjnieapnmpczkhqzhqczqhiciscckvrmehuijkxvzcwljomtfpsvwbygtclgxeselomvamsbormxfbqksliqmiwhmjplojpbyeyyqtcekqdprwcjhmzvuycihstxbjbbnbcduejgumwmkaxrmzmgzroijhgmsjohksmhwvnqkleulemxdafcrtkcdsrxdffqzrxvnmnzyutjyhhdimhbitenovkrcrjbjgyyvnxthsehalkkatlknrxdmjwqbgtmmtkhhjcobhwduinuczgrqdufpqxqwtclmoutzzkcgigaqtxuudlnjhsyarqykulxkgsjfclnmtpdnozjslwduqduitvupgrzmmitqvidpyiemhngumlpcolmjghynaxbmfxfdfisfsyuzncuzojccwqmdxkqyitmpqrsybmftkzvycpzqaduwugbttttbngsfznddzjymktmmklekpzjlfkeyeybtgwyhjcmknlwxgkmryqdppmavxevdezwmvuueygqntplazxtxnwmfjocivxlzyhotdizeqqrkfcmgbferbswkhysexffwotrsbrwuhneossuvxamavklekfiknnibhztkqrezfipzckuzmjahvnliuvshjclsbecuyhtdrleuvatjjvrhkepfajollzdmgfgemcjeppampvvzrmibtxivgxgtyjfeookdsvjhkjtaeobvdjzyghtogzhfiolyewbyrkfcvaearfxwowuwgnmmovrwldwszyqskwwgyaiphflxehvkwjwkeqistfkufaorylxxnhovncutjqdgzbsgrbamimgnmxeniemxlauaepvqhyyicqottqibcqqrnxevdqvqsprzgopnnnwrdmmxfuahlryyoewtwrjricqprfcguaxzpjwuezbpqcpgglzdckunnkcereklhhkwsjqwirnavficqjfvtziglkkeqwrzfdvymnwwhmycrgejrjelkorxaebtcssiwoikdejksjnjsrtjdzupooposqhulcejqmvvteglotximsrrrmjhaywuofobhvkzjivcobqewpmkheeyqngslemblcftxruhzrloudsqsfdzljabwivjmhmzjrsjlpqkgdabhvtbcwdnhdqdllozsdydwtmhkcuqiurrrnstlvitaywnlnodqphmitusyqkimezerrzcpivpiufuhtzziilsehmexxmkblquepwtzslkosgvlfwwohqurdyqckcfxszbkvxlpkvdebarjwnzgetzjtpldsscmrfymekeqhhulkmuizvqfopqhhbmztqnmvmbawfqfonqyzkjafzgvpwewasxvimigyjiwqacensfmfonmnvketoctjbrtozzokhlakpgxjlkqzzdpupvnfyjobaipbjelrrnbgtarfmiglnxhcvvhvgywggojpuuiljltcmmjyusmjrtlmqvhwteebsnsojbzsouxaggbdanmsiaezxmxsqetkrnydozvfvshdbigbxvtgxyhumgmekhoeuohlpeqkxxpaiybftxnqvwmhfynphdvbrcmijeuicjikhjrcghsddgmigzancvrvaddjkxrmhzyqkpttwnyculmcmoqrkhmmmsxwrokapirbakwyxvctipwudolpxablsakjoukffdjfnfdmcgikxghcuusruveowoamjmuopfvekaspgazfgiiadctzzyovmqkxcgnrlixxjpmrqmxwuhzzjuzisnghfsghyuajcxgnmtfeheddlcdwycetrmcweidecfnhcwuqrzvydydqcgmcitvbfxxmgcqkherlcmctlztrzwhddufnndpuaogitxtadkofysykttdspmxugeivhzskpyuivplyazfyrfkmypgocdkkjsittafjyzrwpcwrdcrwjeiitoyxtikddsywudhscbqtfnmjrefakqkfeidatndwhbubgevkfmekbqvuwdwgthozrrzmgekphhsnlnysbecewayvdxdwervxkxkqbuhldzyonygqelvblvwqsbmytjydgiwkedvifvyuwhkashdfdzvvdjwwsuzbexlibisgdtczamauudbzfwuihsgzmfcqdnbgvvoboqcqeckailzpjwzqofsbmutriuzfqpighlxmphoxvooempvaupdjjxcttebltjsburpkadjvtaafpxgmmjxxizrcdsnzbhayyrizutuotwvjvbxhnufxtyzzqparwapvtzwntqdopbzkcwurqbpmzqnpfniepabibxxundleekpihlczmrnljacrnkvemwootmkipvpviedrpfqbyeokgqbwcyapwpdpdnzuloptuuklccmagcmpuplxnlojctafiabtqqolnywpdlrlpeaktljhzxsbqtjyvkuldiyqjxfmkwxzaalwlufmrxqhqzillczokzetwjhiihikhtguolhhjzpbyzpjbqynpejoqqwazekgatdqgmxrlotlnkuoaucrbnftdctdidlkwqadeaupopcezzsbvwtwjyppciunohxmyarcaouodwsfvngtlbpxahoerirfppspplurzoffmakkpsdcwszybnctxuapluecgytfoakbawlrrqunktlmjnfijhxwceyicrprsauuyhmrxgehbrvmhcvskupmynudfjwzlqtmxhdwnkfrbbdidabnvqzvftymfulqmhcloihesnvtqaetbhhyxitecurjoezdleocmwielfzbfdwikfgjocoswhbmlcpzigduelapqcsxfqgswodkqtcoafdohunqasmedpvvrpsygyqenuqfnswqzzeitrawjnaoewjvndtvemztipyvjwlivrrrkrnvgnjmluhbadwerwxzlxwxukfrbyimsxjanvnapwbpbyvsusthsqemcmffmoteldhayubnscfuaamwvignymbcaghnwbtkeakcwlrpqyjqbxpwfxlfjojmurztssdyvpatbtirstjttarazeijsrvokwhcqmancchfwrwqnhlgkeijiquckrqunazgmztoeatcurshbfnljxurqrtailuylteavxqbamdudxdgcsrlpoeffzfpkcnyxraurqvrcanixqoziimzwqfdrkscoicmzcisselxeyfeqvfadsabkfgcrbgijkwioedgiofppmtfiainsipfpvjiwrqhbmsptvxbjdsyrngzooniwennzwvynofqczfvufzektcbhibxyqfwutjvrhvhxdzzcfxxmwdwphxyqavqqbnmwwipfdwcgyelfbirzlnosmnknidflqshoaxufvyotezyfjpewevqridgsglewdnmfkilihbblactltxhyuzrecxxkkdiosvvddinmjoylstwmsfmnhdrjcqwmvecozyrfgersurdnkcbdjnqqqjcuaygveljejclricjdwpsgrtlaibtalvunvciznufwwqqfvzkroezhzlkhodhbabovdzwtteppufwrhikxmkhlvrujwblwebtkardtvulscjjlsuieepsbimkeqxcwzvgznlvigiakrmvlifsovkpfybkobmlcktbzfyhnikzayungzfwwpleqooercrtolxnsvxfejgghatpbblasyeimunewexnpuajszoyiwgpwbeacpavnedcysiwxfjocmywlouulaljdqjghjbxpgznzhmyoxoslmoxdktqmebqwlwewzbohcbglpgznvuyjixmvsehzgzvtxbrwefgijmtytvnopmxxlmuescdybatbelxkqftzsmanhndwbcwnqstjuyjyafzeqypiekkywkngxsebwconvjyrhjrhosogtfpzpbznlotkgalccbpzmpttzkdnftuaylznmaazowjuwxwaaocnfviehnumgqmgbntpnwobszvqyuqzksoppywxoqjwgydjlyybuoconaemwxnipdzhbvernpotnrebpecmsgagifsechotkoaljgdxwtslkzmbdqpwxqchxwmbdawpozaaffzbatnmsbqwsfsrxwyuumxxpumfvohawgewatlsubqogkhegiaauazlunrreytahxyhsjbpmudtenzlvbnrbndxawoewigdhhmasqpfkuaojzbecpyeedbebslxctdjarjvhaatvciedllfalleoplphyojrrsulersetmbvwiyhvfkgrqwgdlnbzyvtsreyiydbmgwiqwslbctsupyycnzcnwxgndgvbjwosuykqswswrkrhlpjaqnonmwapcytoujipjshcqzyimrpxgazzqnoclquqonelyochgjjxlkhxbywkyvlolztypnoecuraithretqpdxmgvkqtgbktcfrssiywepynkfgoaweftsmodejlivekaxeuhcqmtqbfwjuyfaeassoxwzigvorxovsvakjvbmbzvxzcxfcrxspwsxcpovocrgzttlqwxrxvoyxnecfbuuzotcfhxulqcnlxycvxvsukruzxweysayjwcofbitsngfkehgddayptsoqqvrixrtvvibwsuqfawsgfalkcjzwdqovnhgkyzivhjscfijfgdnodyygkaepsulfcrrcszycisepapwjtjxwkxewxklpfywjronkploadspghmrcxqbhnffnstezrbklrxxgjlgofywmknrhdsrtxygrrmqlnawgyvnjgvzltwzflqpriikigszaesluujauepdknfxibwoonffxnpftjhtqbgkvpmqtcvfpacxdznsosxnfptbycwoddayunqormvimyhqzvknphcxwvaxzwncbodybtouhjcuhlmgiyvyfyxevxdvvqzzeexhalbhufderrkzpizxiwbdtgapwcqbinkiamswtvaknnmevlbfieidklshnkkxxuiziawcxwomlariosnwzwzjjuqyecvexddhfbldaaplciuqdhocfolqkeoogmufpanvbaabarpfajxdsmfacsqmyhcftxmizbfcchgezvjzgjeavlaxdtlcanjtmvdlzlobsuhrrtsxcwxxhawnthrtucojzruyckzvgjmxcfwedudnuzanitievwnunkvsuwndfbahixlotobppdcxwmcrkpnzzirwfgbkibssafgvuhkjkohialbkapkczdagkbtuenmhsvcoenarbjzeubqwsaateacqdvkikyrcvttaivrmesqavgwpjxjpznhfirpsrgwnbrnkmgyqaqswypnhoyyplfuvylpqrodtfsfsadkatfyrwmbkmbbyhythjlxdzcwewdywglzbqzumnvyaxbagzvplgpofrmkqpkovvpynpsbeozgnbgvcckpkzdqtpcjemxbknzzesbiicpxdxsizexkgdczuekbsjceyeecyfnghcdnbxxbbwnjzriwxuxpezucfafdfslowqdxnaarxpayilwfftvjvrmartgynroliskvunotersncqsjkptfihhbcnzpfqcnimkczzwesqrugldsbyxjfhnjuoyzecoopzroapuuhnopzticshnzoajjpjkvmzkdabtcsowzdmqehygzgwbuwmfnxsjlrtotwixoyvpbjfgqgosmkvijhqwgifsbaiwwstmqqduzlcrrqjfdepmnfkwnsoqsymrrhfhxjdyufqlybsdihpxnbtctjtoihhzpmgovmprrheriwuselqtplxkvsydafcrukhpwwjqdnjomeiqrylgmkrhdrkfzowtydkxatnfjyvvcelweyeqixxlpjsqxojyuvxwpmbykztpdgljgraopzcgjuwilhifcfbtretedmtiuesaxsrmtvuerspcbxjgfmzytvcyjvhebzgztznouktmciqghcmoqlnjgpiqnpwqqerlefbypumtefzmjlazzpjhhtzukxmacengjogalvvpcxdlkemjvvtcouyyulawppeagmcrhxglwmwhsjqnglqxioxcbayspesvqwoawxlsjkzbtqobdgylxynyyntntllkkzkgyxyhhykcqbethehgcwhcmaqfndpxebddbcmiaxnqytlhxuocrvedxwkkzcigfnjgpxprwcjuvveuruyirfnobaadcilhwwontffqhomelsadmzndgdtjduofezmavcqphvhqlvwzoxeayrgewznztcgadtazxsqfpzvbuucirgjrzpxwqkxmoemuzotwhlbwkzzevbnmjydjpchfoetdkfzjwsgdfjbpkbvfektoiyzzaihpoauzitienqjsckjongwurpibqjbqrihhkbgswryljhflsoahxwbonlrduuwncdxphdhfwgrgnwthntrkrsahermamiqqonfxspcpxtxmqqudmnpdfpmlizoujgnzvuwnjzegsbtkspngccljtyxcznyitlkqggfrfmyantkhianmcbxwzwayjyiekuusbajiojibctlmtaaxbyvcuvcgunkghhaijjrxtbavulkpoknrkkuhiynelzktgkdpvwmoynmuaykqizivlectcpxqojlxvomrnbhnothqtthcupxgvbzxszniiyfgpuxeverlaabtuixozwwuxyurxjahkfnzbmfwtwhvwjostohtmfpmdjdctdkdlzemmjlfhsadbqwvgfqtkffqpkskqepbevpseazlvbqwtotatlvgrcyioquexjhbgxvkzuxuxptqeoszaphttfbiwxmtiuwottaspzpmoieuwxuyobrihuqfrcazyynrrpkgozudjyxhvtgkxzueuklnolpnjeecuoiipclaaliygqkdpehlqsbefqrquwsqmazeemehzofgerjwxtjnnkrtcjjzhcpteoxjdeqcsxixosvlleyyrrttcdsjhkdlacwdgmizlqwzktmxxmyogriiwcsdovrpznugccfntdpfwddvqbkwsuerywuntnetdybfbguwljbjngefhqdakhtlgpybdcwgcxnffhdsthbininvyqxlzqwhfwjiahqcffkmnnfguioiqddrvaeusaexypiywacdgmqtavbufrewwhprjtnbvrbtgjrfltirgamixnedgjbcsqvejjfeaizjtqoflywtzjbwyizhrallshpbqvhsfgdzytgkseftnjporrvktgvmplpfizanhdogxusrzzkoibebublokhczsthcshramgilccedbosmsynmazckiejcyseqzvsodczgeejiudwmcjbekdqsibuiuyavwtjzxlherrolwkaatdeeguwqytystznpuxrtkyxbrloqvqqptxvatrqrmrjgsxvmwofinfjgqxwmondhmyjfrcxetiklfhzdwglllwrcwqqkfttrtfwkhvoceanliorguctgbmixcyhhtcamhxwqsnsyphkxvbfqfywglgdpgmxzpxlywjprwjpmrzlmcteiraoeisuogtnvsawpkniiwxunzicdbglsjczrwtgcmvyewwvgjdofdlvetxhwdvqmxpgtimvwjvdftancgcijpdpicygftwusmalwojtxgyfaysceumplrettfbbyyvrkxegnpbkybpyaicgerwqiclchnfinabxzffmxjwvkovmyzteqrwbtifdxknkmojlvqxsztnmjkyhktmckifgsklrwydiootjfqcgmrsxetfqmtszdyfyksrhqvrellpqdtxrnrprkwhzpzyuzfsmxktphmvszmldgkwtbnkojrpaspvuupgtbbkknffptquzaqsdeetpxiqqxnmteuogvyskfxzzntypmtvqxohvzxnymbsrvgzqhtcmrengxmyjhtbejlbiiqhxrbqmjreqkhfdlskkgiwndzumkvvvmesffksqbwhldopqgbbplxcowyahwxcibwpighbheiujylagobvzaapwyqfupjetfudnbewhorrmetttuqsuqksraanrjskroqgrudbjwsvalqvhhodzyxqaxetjygaowgfqlgzhaxinpduytzytxqvuwrdwxfktoslzanhbkrqbwlaqmaljqjgpallxtyshqdgqqptcgkedxevjgivyrdnrobojzgqrmzjcqffxrlcorzzxhwvbzjsqmjznrsgdrpghwngnykdpldmtzjwbsorgzqtrizubrclpprdpegeplskncgowxfdwxyniykjrmugeoltssahfsusuagrznwwlultuvclkzuonfjfxjofcixylermrnieiuxcrcqbbkropbtpkjuournhxetrsevcatervwvwgmmynfnyqjokabtagnratocthikefhcnuolhvahmjwymzsmhhfhatlvdwhhdpkqjaesweakoyicxcofltonociryqzbhltqlzijektuieyiimpuhdjxhspfkqirbejodrajcvfmzdwkrlgarpyyjnetdowoikdejksjnjsrtjdzupooposqhulcejqmvvteglotximsrrrmjhaywuofobhvkzjivcobqewpmkheeyqngslemblcftxruhzrloudsqsfdzljabwivjmhmzjrsjlpqkgdabhvtbcwdnhdqdllozsdydwtmhkcuqiurrrnstlvitaywnlnodqphmitusyqkimezerrzcpivpiufuhtzziilsehmexxmkblquepwtzslkosgvlfwwohqurdyqckcfxszbkvxlpkvdebarjwnzgetzjtpldsscmrfymekeqhhulkmuizvqfopqhhbmztqnmvmbawfqfonqyzkjafzgvpwewasxvimigyjiwqacensfmfonmnvketoctjbrtozzokhlakpgxjlkqzzdpupvnfyjobaipbjelrrnbgtarfmiglnxhcvvhvgywggojpuuiljltcmmjyusmjrtlmqvhwteebsnsojbzsouxaggbdanmsiaezxmxsqetkrnydozvfvshdbigbxvtgxyhumgmekhoeuohlpeqkxxpaiybftxnqvwmhfynphdvbrcmijeuicjikhjrcghsddgmigzancvrvaddjkxrmhzyqkpttwnyculmcmoqrkhmmmsxwrokapirbakwyxvctipwudolpxablsakjoukffdjfnfdmcgikxghcuusruveowoamjmuopfvekaspgazfgiiadctzzyovmqkxcgnrlixxjpmrqmxwuhzzjuzisnghfsghyuajcxgnmtfeheddlcdwycetrmcweidecfnhcwuqrzvydydqcgmcitvbfxxmgcqkherlcmctlztrzwhddufnndpuaogitxtadkofysykttdspmxugeivhzskpyuivplyazfyrfkmypgocdkkjsittafjyzrwpcwrdcrwjeiitoyxtikddsywudhscbqtfnmjrefakqkfeidatndwhbubgevkfmekbqvuwdwgthozrrzmgekphhsnlnysbecewayvdxdwervxkxkqbuhldzyonygqelvblvwqsbmytjydgiwkedvifvyuwhkashdfdzvvdjwwsuzbexlibisgdtczamauudbzfwuihsgzmfcqdnbgvvoboqcqeckailzpjwzqofsbmutriuzfqpighlxmphoxvooempvaupdjjxcttebltjsburpkadjvtaafpxgmmjxxizrcdsnzbhayyrizutuotwvjvbxhnufxtyzzqparwapvtzwntqdopbzkcwurqbpmzqnpfniepabibxxundleekpihlczmrnljacrnkvemwootmkipvpviedrpfqbyeokgqbwcyapwpdpdnzuloptuuklccmagcmpuplxnlojctafiabtqqolnywpdlrlpeaktljhzxsbqtjyvkuldiyqjxfmkwxzaalwlufmrxqhqzillczokzetwjhiihikhtguolhhjzpbyzpjbqynpejoqqwazekgatdqgmxrlotlnkuoaucrbnftdctdidlkwqadeaupopcezzsbvwtwjyppciunohxmyarcaouodwsfvngtlbpxahoerirfppspplurzoffmakkpsdcwszybnctxuapluecgytfoakbawlrrqunktlmjnfijhxwceyicrprsauuyhmrxgehbrvmhcvskupmynudfjwzlqtmxhdwnkfrbbdidabnvqzvftymfulqmhcloihesnvtqaetbhhyxitecurjoezdleocmwielfzbfdwikfgjocoswhbmlcpzigduelapqcsxfqgswodkqtcoafdohunqasmedpvvrpsygyqenuqfnswqzzeitrawjnaoewjvndtvemztipyvjwlivrrrkrnvgnjmluhbadwerwxzlxwxukfrbyimsxjanvnapwbpbyvsusthsqemcmffmoteldhayubnscfuaamwvignymbcaghnwbtkeakcwlrpqyjqbxpwfxlfjojmurztssdyvpatbtirstjttarazeijsrvokwhcqmancchfwrwqnhlgkeijiquckrqunazgmztoeatcurshbfnljxurqrtailuylteavxqbamdudxdgcsrlpoeffzfpkcnyxraurqvrcanixqoziimzwqfdrkscoicmzcisselxeyfeqvfadsabkfgcrbgijkwioedgiofppmtfiainsipfpvjiwrqhbmsptvxbjdsyrngzooniwennzwvynofqczfvufzektcbhibxyqfwutjvrhvhxdzzcfxxmwdwphxyqavqqbnmwwipfdwcgyelfbirzlnosmnknidflqshoaxufvyotezyfjpewevqridgsglewdnmfkilihbblactltxhyuzrecxxkkdiosvvddinmjoylstwmsfmnhdrjcqwmvecozyrfgersurdnkcbdjnqqqjcuaygveljejclricjdwpsgrtlaibtalvunvciznufwwqqfvzkroezhzlkhodhbabovdzwtteppufwrhikxmkhlvrujwblwebtkardtvulscjjlsuieepsbimkeqxcwzvgznlvigiakrmvlifsovkpfybkobmlcktbzfyhnikzayungzfwwpleqooercrtolxnsvxfejgghatpbblasyeimunewexnpuajszoyiwgpwbeacpavnedcysiwxfjocmywlouulaljdqjghjbxpgznzhmyoxoslmoxdktqmebqwlwewzbohcbglpgznvuyjixmvsehzgzvtxbrwefgijmtytvnopmxxlmuescdybatbelxkqftzsmanhndwbcwnqstjuyjyafzeqypiekkywkngxsebwconvjyrhjrhosogtfpzpbznlotkgalccbpzmpttzkdnftuaylznmaazowjuwxwaaocnfviehnumgqmgbntpnwobszvqyuqzksoppywxoqjwgydjlyybuoconaemwxnipdzhbvernpotnrebpecmsgagifsechotkoaljgdxwtslkzmbdqpwxqchxwmbdawpozaaffzbatnmsbqwsfsrxwyuumxxpumfvohawgewatlsubqogkhegiaauazlunrreytahxyhsjbpmudtenzlvbnrbndxawoewigdhhmasqpfkuaojzbecpyeedbebslxctdjarjvhaatvciedllfalleoplphyojrrsulersetmbvwiyhvfkgrqwgdlnbzyvtsreyiydbmgwiqwslbctsupyycnzcnwxgndgvbjwosuykqswswrkrhlpjaqnonmwapcytoujipjshcqzyimrpxgazzqnoclquqonelyochgjjxlkhxbywkyvlolztypnoecuraithretqpdxmgvkqtgbktcfrssiywepynkfgoaweftsmodejlivekaxeuhcqmtqbfwjuyfaeassoxwzigvorxovsvakjvbmbzvxzcxfcrxspwsxcpovocrgzttlqwxrxvoyxnecfbuuzotcfhxulqcnlxycvxvsukruzxweysayjwcofbitsngfkehgddayptsoqqvrixrtvvibwsuqfawsgfalkcjzwdqovnhgkyzivhjscfijfgdnodyygkaepsulfcrrcszycisepapwjtjxwkxewxklpfywjronkploadspghmrcxqbhnffnstezrbklrxxgjlgofywmknrhdsrtxygrrmqlnawgyvnjgvzltwzflqpriikigszaesluujauepdknfxibwoonffxnpftjhtqbgkvpmqtcvfpacxdznsosxnfptbycwoddayunqormvimyhqzvknphcxwvaxzwncbodybtouhjcuhlmgiyvyfyxevxdvvqzzeexhalbhufderrkzpizxiwbdtgapwcqbinkiamswtvaknnmevlbfieidklshnkkxxuiziawcxwomlariosnwzwzjjuqyecvexddhfbldaaplciuqdhocfolqkeoogmufpanvbaabarpfajxdsmfacsqmyhcftxmizbfcchgezvjzgjeavlaxdtlcanjtmvdlzlobsuhrrtsxcwxxhawnthrtucojzruyckzvgjmxcfwedudnuzanitievwnunkvsuwndfbahixlotobppdcxwmcrkpnzzirwfgbkibssafgvuhkjkohialbkapkczdagkbtuenmhsvcoenarbjzeubqwsaateacqdvkikyrcvttaivrmesqavgwpjxjpznhfirpsrgwnbrnkmgyqaqswypnhoyyplfuvylpqrodtfsfsadkatfyrwmbkmbbyhythjlxdzcwewdywglzbqzumnvyaxbagzvplgpofrmkqpkovvpynpsbeozgnbgvcckpkzdqtpcjemxbknzzesbiicpxdxsizexkgdczuekbsjceyeecyfnghcdnbxxbbwnjzriwxuxpezucfafdfslowqdxnaarxpayilwfftvjvrmartgynroliskvunotersncqsjkptfihhbcnzpfqcnimkczzwesqrugldsbyxjfhnjuoyzecoopzroapuuhnopzticshnzoajjpjkvmzkdabtcsowzdmqehygzgwbuwmfnxsjlrtotwixoyvpbjfgqgosmkvijhqwgifsbaiwwstmqqduzlcrrqjfdepmnfkwnsoqsymrrhfhxjdyufqlybsdihpxnbtctjtoihhzpmgovmprrheriwuselqtplxkvsydafcrukhpwwjqdnjomeiqrylgmkrhdrkfzowtydkxatnfjyvvcelweyeqixxlpjsqxojyuvxwpmbykztpdgljgraopzcgjuwilhifcfbtretedmtiuesaxsrmtvuerspcbxjgfmzytvcyjvhebzgztznouktmciqghcmoqlnjgpiqnpwqqerlefbypumtefzmjlazzpjhhtzukxmacengjogalvvpcxdlkemjvvtcouyyulawppeagmcrhxglwmwhsjqnglqxioxcbayspesvqwoawxlsjkzbtqobdgylxynyyntntllkkzkgyxyhhykcqbethehgcwhcmaqfndpxebddbcmiaxnqytlhxuocrvedxwkkzcigfnjgpxprwcjuvveuruyirfnobaadcilhwwontffqhomelsadmzndgdtjduofezmavcqphvhqlvwzoxeayrgewznztcgadtazxsqfpzvbuucirgjrzpxwqkxmoemuzotwhlbwkzzevbnmjydjpchfoetdkfzjwsgdfjbpkbvfektoiyzzaihpoauzitienqjsckjongwurpibqjbqrihhkbgswryljhflsoahxwbonlrduuwncdxphdhfwgrgnwthntrkrsahermamiqqonfxspcpxtxmqqudmnpdfpmlizoujgnzvuwnjzegsbtkspngccljtyxcznyitlkqggfrfmyantkhianmcbxwzwayjyiekuusbajiojibctlmtaaxbyvcuvcgunkghhaijjrxtbavulkpoknrkkuhiynelzktgkdpvwmoynmuaykqizivlectcpxqojlxvomrnbhnothqtthcupxgvbzxszniiyfgpuxeverlaabtuixozwwuxyurxjahkfnzbmfwtwhvwjostohtmfpmdjdctdkdlzemmjlfhsadbqwvgfqtkffqpkskqepbevpseazlvbqwtotatlvgrcyioquexjhbgxvkzuxuxptqeoszaphttfbiwxmtiuwottaspzpmoieuwxuyobrihuqfrcazyynrrpkgozudjyxhvtgkxzueuklnolpnjeecuoiipclaaliygqkdpehlqsbefqrquwsqmazeemehzofgerjwxtjnnkrtcjjzhcpteoxjdeqcsxixosvlleyyrrttcdsjhkdlacwdgmizlqwzktmxxmyogriiwcsdovrpznugccfntdpfwddvqbkwsuerywuntnetdybfbguwljbjngefhqdakhtlgpybdcwgcxnffhdsthbininvyqxlzqwhfwjiahqcffkmnnfguioiqddrvaeusaexypiywacdgmqtavbufrewwhprjtnbvrbtgjrfltirgamixnedgjbcsqvejjfeaizjtqoflywtzjbwyizhrallshpbqvhsfgdzytgkseftnjporrvktgvmplpfizanhdogxusrzzkoibebublokhczsthcshramgilccedbosmsynmazckiejcyseqzvsodczgeejiudwmcjbekdqsibuiuyavwtjzxlherrolwkaatdeeguwqytystznpuxrtkyxbrloqvqqptxvatrqrmrjgsxvmwofinfjgqxwmondhmyjfrcxetiklfhzdwglllwrcwqqkfttrtfwkhvoceanliorguctgbmixcyhhtcamhxwqsnsyphkxvbfqfywglgdpgmxzpxlywjprwjpmrzlmcteiraoeisuogtnvsawpkniiwxunzicdbglsjczrwtgcmvyewwvgjdofdlvetxhwdvqmxpgtimvwjvdftancgcijpdpicygftwusmalwojtxgyfaysceumplrettfbbyyvrkxegnpbkybpyaicgerwqiclchnfinabxzffmxjwvkovmyzteqrwbtifdxknkmojlvqxsztnmjkyhktmckifgsklrwydiootjfqcgmrsxetfqmtszdyfyksrhqvrellpqdtxrnrprkwhzpzyuzfsmxktphmvszmldgkwtbnkojrpaspvuupgtbbkknffptquzaqsdeetpxiqqxnmteuogvyskfxzzntypmtvqxohvzxnymbsrvgzqhtcmrengxmyjhtbejlbiiqhxrbqmjreqkhfdlskkgiwndzumkvvvmesffksqbwhldopqgbbplxcowyahwxcibwpighbheiujylagobvzaapwyqfupjetfudnbewhorrmetttuqsuqksraanrjskroqgrudbjwsvalqvhhodzyxqaxetjygaowgfqlgzhaxinpduytzytxqvuwrdwxfktoslzanhbkrqbwlaqmaljqjgpallxtyshqdgqqptcgkedxevjgivyrdnrobojzgqrmzjcqffxrlcorzzxhwvbzjsqmjznrsgdrpghwngnykdpldmtzjwbsorgzqtrizubrclpprdpegeplskncgowxfdwxyniykjrmugeoltssahfsusuagrznwwlultuvclkzuonfjfxjofcixylermrnieiuxcrcqbbkropbtpkjuournhxetrsevcatervwvwgmmynfnyqjokabtagnratocthikefhcnuolhvahmjwymzsmhhfhatlvdwhhdpkqjaesweakoyicxcofltonociryqzbhltqlzijektuieyiimpuhdjxhspfkqirbejodrajcvfmzdwkrlgarpyyjnetdo".to_string()), "woikdejksjnjsrtjdzupooposqhulcejqmvvteglotximsrrrmjhaywuofobhvkzjivcobqewpmkheeyqngslemblcftxruhzrloudsqsfdzljabwivjmhmzjrsjlpqkgdabhvtbcwdnhdqdllozsdydwtmhkcuqiurrrnstlvitaywnlnodqphmitusyqkimezerrzcpivpiufuhtzziilsehmexxmkblquepwtzslkosgvlfwwohqurdyqckcfxszbkvxlpkvdebarjwnzgetzjtpldsscmrfymekeqhhulkmuizvqfopqhhbmztqnmvmbawfqfonqyzkjafzgvpwewasxvimigyjiwqacensfmfonmnvketoctjbrtozzokhlakpgxjlkqzzdpupvnfyjobaipbjelrrnbgtarfmiglnxhcvvhvgywggojpuuiljltcmmjyusmjrtlmqvhwteebsnsojbzsouxaggbdanmsiaezxmxsqetkrnydozvfvshdbigbxvtgxyhumgmekhoeuohlpeqkxxpaiybftxnqvwmhfynphdvbrcmijeuicjikhjrcghsddgmigzancvrvaddjkxrmhzyqkpttwnyculmcmoqrkhmmmsxwrokapirbakwyxvctipwudolpxablsakjoukffdjfnfdmcgikxghcuusruveowoamjmuopfvekaspgazfgiiadctzzyovmqkxcgnrlixxjpmrqmxwuhzzjuzisnghfsghyuajcxgnmtfeheddlcdwycetrmcweidecfnhcwuqrzvydydqcgmcitvbfxxmgcqkherlcmctlztrzwhddufnndpuaogitxtadkofysykttdspmxugeivhzskpyuivplyazfyrfkmypgocdkkjsittafjyzrwpcwrdcrwjeiitoyxtikddsywudhscbqtfnmjrefakqkfeidatndwhbubgevkfmekbqvuwdwgthozrrzmgekphhsnlnysbecewayvdxdwervxkxkqbuhldzyonygqelvblvwqsbmytjydgiwkedvifvyuwhkashdfdzvvdjwwsuzbexlibisgdtczamauudbzfwuihsgzmfcqdnbgvvoboqcqeckailzpjwzqofsbmutriuzfqpighlxmphoxvooempvaupdjjxcttebltjsburpkadjvtaafpxgmmjxxizrcdsnzbhayyrizutuotwvjvbxhnufxtyzzqparwapvtzwntqdopbzkcwurqbpmzqnpfniepabibxxundleekpihlczmrnljacrnkvemwootmkipvpviedrpfqbyeokgqbwcyapwpdpdnzuloptuuklccmagcmpuplxnlojctafiabtqqolnywpdlrlpeaktljhzxsbqtjyvkuldiyqjxfmkwxzaalwlufmrxqhqzillczokzetwjhiihikhtguolhhjzpbyzpjbqynpejoqqwazekgatdqgmxrlotlnkuoaucrbnftdctdidlkwqadeaupopcezzsbvwtwjyppciunohxmyarcaouodwsfvngtlbpxahoerirfppspplurzoffmakkpsdcwszybnctxuapluecgytfoakbawlrrqunktlmjnfijhxwceyicrprsauuyhmrxgehbrvmhcvskupmynudfjwzlqtmxhdwnkfrbbdidabnvqzvftymfulqmhcloihesnvtqaetbhhyxitecurjoezdleocmwielfzbfdwikfgjocoswhbmlcpzigduelapqcsxfqgswodkqtcoafdohunqasmedpvvrpsygyqenuqfnswqzzeitrawjnaoewjvndtvemztipyvjwlivrrrkrnvgnjmluhbadwerwxzlxwxukfrbyimsxjanvnapwbpbyvsusthsqemcmffmoteldhayubnscfuaamwvignymbcaghnwbtkeakcwlrpqyjqbxpwfxlfjojmurztssdyvpatbtirstjttarazeijsrvokwhcqmancchfwrwqnhlgkeijiquckrqunazgmztoeatcurshbfnljxurqrtailuylteavxqbamdudxdgcsrlpoeffzfpkcnyxraurqvrcanixqoziimzwqfdrkscoicmzcisselxeyfeqvfadsabkfgcrbgijkwioedgiofppmtfiainsipfpvjiwrqhbmsptvxbjdsyrngzooniwennzwvynofqczfvufzektcbhibxyqfwutjvrhvhxdzzcfxxmwdwphxyqavqqbnmwwipfdwcgyelfbirzlnosmnknidflqshoaxufvyotezyfjpewevqridgsglewdnmfkilihbblactltxhyuzrecxxkkdiosvvddinmjoylstwmsfmnhdrjcqwmvecozyrfgersurdnkcbdjnqqqjcuaygveljejclricjdwpsgrtlaibtalvunvciznufwwqqfvzkroezhzlkhodhbabovdzwtteppufwrhikxmkhlvrujwblwebtkardtvulscjjlsuieepsbimkeqxcwzvgznlvigiakrmvlifsovkpfybkobmlcktbzfyhnikzayungzfwwpleqooercrtolxnsvxfejgghatpbblasyeimunewexnpuajszoyiwgpwbeacpavnedcysiwxfjocmywlouulaljdqjghjbxpgznzhmyoxoslmoxdktqmebqwlwewzbohcbglpgznvuyjixmvsehzgzvtxbrwefgijmtytvnopmxxlmuescdybatbelxkqftzsmanhndwbcwnqstjuyjyafzeqypiekkywkngxsebwconvjyrhjrhosogtfpzpbznlotkgalccbpzmpttzkdnftuaylznmaazowjuwxwaaocnfviehnumgqmgbntpnwobszvqyuqzksoppywxoqjwgydjlyybuoconaemwxnipdzhbvernpotnrebpecmsgagifsechotkoaljgdxwtslkzmbdqpwxqchxwmbdawpozaaffzbatnmsbqwsfsrxwyuumxxpumfvohawgewatlsubqogkhegiaauazlunrreytahxyhsjbpmudtenzlvbnrbndxawoewigdhhmasqpfkuaojzbecpyeedbebslxctdjarjvhaatvciedllfalleoplphyojrrsulersetmbvwiyhvfkgrqwgdlnbzyvtsreyiydbmgwiqwslbctsupyycnzcnwxgndgvbjwosuykqswswrkrhlpjaqnonmwapcytoujipjshcqzyimrpxgazzqnoclquqonelyochgjjxlkhxbywkyvlolztypnoecuraithretqpdxmgvkqtgbktcfrssiywepynkfgoaweftsmodejlivekaxeuhcqmtqbfwjuyfaeassoxwzigvorxovsvakjvbmbzvxzcxfcrxspwsxcpovocrgzttlqwxrxvoyxnecfbuuzotcfhxulqcnlxycvxvsukruzxweysayjwcofbitsngfkehgddayptsoqqvrixrtvvibwsuqfawsgfalkcjzwdqovnhgkyzivhjscfijfgdnodyygkaepsulfcrrcszycisepapwjtjxwkxewxklpfywjronkploadspghmrcxqbhnffnstezrbklrxxgjlgofywmknrhdsrtxygrrmqlnawgyvnjgvzltwzflqpriikigszaesluujauepdknfxibwoonffxnpftjhtqbgkvpmqtcvfpacxdznsosxnfptbycwoddayunqormvimyhqzvknphcxwvaxzwncbodybtouhjcuhlmgiyvyfyxevxdvvqzzeexhalbhufderrkzpizxiwbdtgapwcqbinkiamswtvaknnmevlbfieidklshnkkxxuiziawcxwomlariosnwzwzjjuqyecvexddhfbldaaplciuqdhocfolqkeoogmufpanvbaabarpfajxdsmfacsqmyhcftxmizbfcchgezvjzgjeavlaxdtlcanjtmvdlzlobsuhrrtsxcwxxhawnthrtucojzruyckzvgjmxcfwedudnuzanitievwnunkvsuwndfbahixlotobppdcxwmcrkpnzzirwfgbkibssafgvuhkjkohialbkapkczdagkbtuenmhsvcoenarbjzeubqwsaateacqdvkikyrcvttaivrmesqavgwpjxjpznhfirpsrgwnbrnkmgyqaqswypnhoyyplfuvylpqrodtfsfsadkatfyrwmbkmbbyhythjlxdzcwewdywglzbqzumnvyaxbagzvplgpofrmkqpkovvpynpsbeozgnbgvcckpkzdqtpcjemxbknzzesbiicpxdxsizexkgdczuekbsjceyeecyfnghcdnbxxbbwnjzriwxuxpezucfafdfslowqdxnaarxpayilwfftvjvrmartgynroliskvunotersncqsjkptfihhbcnzpfqcnimkczzwesqrugldsbyxjfhnjuoyzecoopzroapuuhnopzticshnzoajjpjkvmzkdabtcsowzdmqehygzgwbuwmfnxsjlrtotwixoyvpbjfgqgosmkvijhqwgifsbaiwwstmqqduzlcrrqjfdepmnfkwnsoqsymrrhfhxjdyufqlybsdihpxnbtctjtoihhzpmgovmprrheriwuselqtplxkvsydafcrukhpwwjqdnjomeiqrylgmkrhdrkfzowtydkxatnfjyvvcelweyeqixxlpjsqxojyuvxwpmbykztpdgljgraopzcgjuwilhifcfbtretedmtiuesaxsrmtvuerspcbxjgfmzytvcyjvhebzgztznouktmciqghcmoqlnjgpiqnpwqqerlefbypumtefzmjlazzpjhhtzukxmacengjogalvvpcxdlkemjvvtcouyyulawppeagmcrhxglwmwhsjqnglqxioxcbayspesvqwoawxlsjkzbtqobdgylxynyyntntllkkzkgyxyhhykcqbethehgcwhcmaqfndpxebddbcmiaxnqytlhxuocrvedxwkkzcigfnjgpxprwcjuvveuruyirfnobaadcilhwwontffqhomelsadmzndgdtjduofezmavcqphvhqlvwzoxeayrgewznztcgadtazxsqfpzvbuucirgjrzpxwqkxmoemuzotwhlbwkzzevbnmjydjpchfoetdkfzjwsgdfjbpkbvfektoiyzzaihpoauzitienqjsckjongwurpibqjbqrihhkbgswryljhflsoahxwbonlrduuwncdxphdhfwgrgnwthntrkrsahermamiqqonfxspcpxtxmqqudmnpdfpmlizoujgnzvuwnjzegsbtkspngccljtyxcznyitlkqggfrfmyantkhianmcbxwzwayjyiekuusbajiojibctlmtaaxbyvcuvcgunkghhaijjrxtbavulkpoknrkkuhiynelzktgkdpvwmoynmuaykqizivlectcpxqojlxvomrnbhnothqtthcupxgvbzxszniiyfgpuxeverlaabtuixozwwuxyurxjahkfnzbmfwtwhvwjostohtmfpmdjdctdkdlzemmjlfhsadbqwvgfqtkffqpkskqepbevpseazlvbqwtotatlvgrcyioquexjhbgxvkzuxuxptqeoszaphttfbiwxmtiuwottaspzpmoieuwxuyobrihuqfrcazyynrrpkgozudjyxhvtgkxzueuklnolpnjeecuoiipclaaliygqkdpehlqsbefqrquwsqmazeemehzofgerjwxtjnnkrtcjjzhcpteoxjdeqcsxixosvlleyyrrttcdsjhkdlacwdgmizlqwzktmxxmyogriiwcsdovrpznugccfntdpfwddvqbkwsuerywuntnetdybfbguwljbjngefhqdakhtlgpybdcwgcxnffhdsthbininvyqxlzqwhfwjiahqcffkmnnfguioiqddrvaeusaexypiywacdgmqtavbufrewwhprjtnbvrbtgjrfltirgamixnedgjbcsqvejjfeaizjtqoflywtzjbwyizhrallshpbqvhsfgdzytgkseftnjporrvktgvmplpfizanhdogxusrzzkoibebublokhczsthcshramgilccedbosmsynmazckiejcyseqzvsodczgeejiudwmcjbekdqsibuiuyavwtjzxlherrolwkaatdeeguwqytystznpuxrtkyxbrloqvqqptxvatrqrmrjgsxvmwofinfjgqxwmondhmyjfrcxetiklfhzdwglllwrcwqqkfttrtfwkhvoceanliorguctgbmixcyhhtcamhxwqsnsyphkxvbfqfywglgdpgmxzpxlywjprwjpmrzlmcteiraoeisuogtnvsawpkniiwxunzicdbglsjczrwtgcmvyewwvgjdofdlvetxhwdvqmxpgtimvwjvdftancgcijpdpicygftwusmalwojtxgyfaysceumplrettfbbyyvrkxegnpbkybpyaicgerwqiclchnfinabxzffmxjwvkovmyzteqrwbtifdxknkmojlvqxsztnmjkyhktmckifgsklrwydiootjfqcgmrsxetfqmtszdyfyksrhqvrellpqdtxrnrprkwhzpzyuzfsmxktphmvszmldgkwtbnkojrpaspvuupgtbbkknffptquzaqsdeetpxiqqxnmteuogvyskfxzzntypmtvqxohvzxnymbsrvgzqhtcmrengxmyjhtbejlbiiqhxrbqmjreqkhfdlskkgiwndzumkvvvmesffksqbwhldopqgbbplxcowyahwxcibwpighbheiujylagobvzaapwyqfupjetfudnbewhorrmetttuqsuqksraanrjskroqgrudbjwsvalqvhhodzyxqaxetjygaowgfqlgzhaxinpduytzytxqvuwrdwxfktoslzanhbkrqbwlaqmaljqjgpallxtyshqdgqqptcgkedxevjgivyrdnrobojzgqrmzjcqffxrlcorzzxhwvbzjsqmjznrsgdrpghwngnykdpldmtzjwbsorgzqtrizubrclpprdpegeplskncgowxfdwxyniykjrmugeoltssahfsusuagrznwwlultuvclkzuonfjfxjofcixylermrnieiuxcrcqbbkropbtpkjuournhxetrsevcatervwvwgmmynfnyqjokabtagnratocthikefhcnuolhvahmjwymzsmhhfhatlvdwhhdpkqjaesweakoyicxcofltonociryqzbhltqlzijektuieyiimpuhdjxhspfkqirbejodrajcvfmzdwkrlgarpyyjnetdo".to_string());
        assert_eq!(Solution::longest_dup_substring("banana".to_string()), "ana".to_string());
        assert_eq!(Solution::longest_dup_substring("abcd".to_string()), "".to_string());
    }
}