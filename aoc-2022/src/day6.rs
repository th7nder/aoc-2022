use std::collections::HashMap;


pub const INPUT_SIGNAL: &str = "pqffvllhrhthvhshhpnhpnpqpvpvrpvpwvwjjdssmcsmccjvjmjjwnjwjwhjwwwzswwhvhwwlvvlbvbtbzbfbzbtbqbgbpbggwzggvjjdpdffbmffntncchtccbcffcjfjnjfnntssvtstzssmnnhrhlhbbwfwjfwjfwwbhhfhmmpsssbnssssfzzfpffdrdpdqqvnncjjgrjjmhhpqqcjqcjjzdzzpvvprrlglrrcmcqqtltdltddswsrrzzwgzzgssczcmzzmgmwgmggwwzttpccmcsmmvfvnvppzlzvzllgclggpfggfnfrfvrvwwvhwwvgwwrbbgfglflblzblzbznzhzffplffnrrcqqsgsvshvhlldhhvnhhmdddnssdvdwdwccggmddsmswwtctdtqqjsshhjzzdpdmpdmppjtjwjswjsjjjsdjjtrtbrbjjwwvnvppqphqhwhcwhwbbpgbbnhbnhhswwdswwlcczdztzbbbnwwtmmpvvgjjqgqdqzzdjdpjjnnffhccscvvchhbmbcbffpdpggvdvttpvpqqhggdtdhtdhhmghmgggzwgwrgwggwlggvpggcfcttzmtmgmvgmmpqmqlmqllsqqjbjwjsszczlzrzgrzzhshlhjjwttwnntbtjtjpplccqrqhrhssbmbttrddfvfwwjcwcvwcwwvpvggqwgwjgwgccvqqcmqqtqnqpnqnffdqfqhhqnhnmhmvhmhwwfrwrggnmmmcnmmgsszmzlmzmddcwwthtssgjsgjjgpgnppdqqcgqggzjgjngnrnggvffgddvtvctcftcftfnnnnhssbgsgwwthtqtltftqtnqttsrtrggwcgwcwmwgmgvmmzrmzrzjzmmcclmmtjmjhmmlhlwlppnpccbbrlrqrcrjrdrlrnngmnmvmcmzczztbblglccvzvppzspsddrzzlsllfzfsspnpdnpnvvvgmmpccmpcpgcpcwcddtmddgwgngqqcpqqlhqqczqqbvqqgdqgqmgmlmmvrrgfgzffbccldcdmmcmcgcngnghngngdngdndcncbbpqbbphbphpccpcwwjswwfttbqbsszccrbbdndsdrdqrqjrjjbmbtbdbbgbvgvcggwdwcccttqccnffjpjqqzpzlzvlljhhschhzlhhfhcfhchvcvtvtgvgzvzrvrdrgrwrjwjljhljlssszsqzsswhhmlhlrhlhzzgghjhzjjcllwrwtrrbdbrbnrnprnrffjvvphhvbbqbbscstsmslmlvmvrmmvvngnlnzzwqzzjqqsqbqrrtmrtmmfgmgrgjjtmjmrrddmrrqmrrjmjqmqnqmmcmlmfmffcgcclplffzvzwvzzjtztftqftqffjjpwjjbpjjggzdgzgwzzfrfvvhfvvwcvvbccfcvffpcpgpbbqhbbhmmzfmfvvnjvnjjhzhqqmffndndmmzhmmqnnlglvvjhjddvggqwgwdgwdggqbqgqrqlrrtptsswlssqwssbdsdrsddjsszjszjjpnjnvnjvnnmznmnddccpwwhshzzcfcqcwcddsjjmnjmjljwwgmglllqlhhctcvvqrvrrhfrrbcrrfbrfbrfrqffbwbqwbbjggsjjjnqqrqsqhhwnhnshnhhdjjqfqpqmmqgqgqggzmmnncrrpgglgqlqclqlsqqwnnfntnzttrnnmtmvvfppbrprzpzzdzvvtctnncpclpccsbbswwcscjssvhvhhqggzmgmqgmgwwgcwgccrllzhzzlzlbljbllmqqpjqqhrhqhjhbbjmjmhmddmwmcmvvmbbmvbmmznnwvwlwtllhwlwgwpgplgpgmgngjgglbglgmllvvlttgrrrlsrllghlggjdjwwfjwfjfvhjmgqnwhwpbdtzrphsqbmmvscslhbdzffsfshgsdjbqbwlgmrtschcnfhdlnndsvpwmwttfglpghhznmgfcjsdlwhnmfqvmpvhgpnnwtjfztbmtprqhsqtjwzhwcqjtjbtqwlcldnvggrwddmpllwnrqwdljwzfzqwcdwgqwvnthnrpcsfwrmqvbzjvzqnmdnfgtbzgtnrvblfwmhdsddgbffnjzvjzfpwglctpqhnqdvtblcchrlmndzhlsczgnsmnbwgnjngnjtlrdpfhqjrwcrqvcpspbtwcvgvvmpnwqjjpdpnslmcrcjnjmhqmrmfbcmrcmpbcbhpcvwqwflljfpgdvqhgdwgcphjqfnqzjjpsqnbtfzhftjtfcbhhcmmlwcfznsflfpphprrgvqwfgjcwfgjfsghzcbqrldwrjlzlbjhpgrbmgdpgzmfsqsphqbbslwwpzspccrhcfrgcjlfwhlcmzdcltbbpcrzglqgqntpwtmgstqlmcsqqbsqgmsmfznwcrfdgvsmnfqmwtsvqvlhwwjlrlhnsvcnrtwwmrjcgfncvlrcqrllndlvmrjpfjpgrrjcwhsqvlbtnlqgwjjqzwcvtvlnfnmqqshbcnqtcbvnwtwbfdgqmvnpmjhlsfdntfwwntvsrrsmspzqmglfnprjtdbmbgnplzzclsjpnzwdhcbhpfnqrgmgqtpfhgnfbqhrpmznbrshjhntzctslwhtgtjvpqhntmchhtncfjmbzcgnpcbpmldrtnpvrzqfftbjjcjlpwwgvmnstjghftcczjzfsftgzpfhbspqmrbfhcdfmqbrgrbsmjvgpbrnvbblwwvqzzpmqrspzvzppjfbgfftdvsdvmrjzhfslptzmgndnqqgmrrfnbbpvbmvpngwjhzvfbwfnzlrgwffvjsfdldfgchfjmnzfnzhwrwttrzlrhmnwvjjdqfmbpfllhrgmddjgnwjnbqwjnslcrdjrmnldcpsgzjpdhrpdfwhbvwhwnhcsmwcwstvqrcrqsnvjrzljfgbljfszchbsqnldgntvcscwqqmpnlwtlfmswtmvrlpzgbrjhtgjgpnhggnprpvwfqpjffqhtfvpnrptgrtwzzlvplgnfjmqphgmnssccrdndqgpljtwtntshrpgsjcdrpmccjnjdgmpmzbfhqjzphcswtwvvqcrwsjhtdqgrhqjmjjcrblpswcblnpzvfztqtbpgjcgngqmwrjtlmhvlsbmrdzwlgqlfqcqnsnjcnddssqbftjvnlgcwwfcgdpdmqrdsjmcnzrfrpnvjmbsltpzwjhjzqqvbgrltczbgvcpwdzqsvhddsbjgjgcmnldrfhnhddlvjcvsnghprjwlghhtghldcqsdcdgnmbcjglvjjvvlbhzczlmjsdqtdpzdtvfztgsdfjsdtfchvzcgvhjnnncmsrfvvmcsjjdftmlpczgvtwngssqmzlmsrrsrbhhhrnwqhmpcdvqmdsvvtsgsqfdcpgsdgzvmbzpbpgtcbshnvdzlmpnwmqrvnmrjprmvppjwfbjhlhzsfhqqzmpbclqvsvfrcqwprrcvqcbbwvnqfwnrgjhlwmgzpfspqrvqrhmqnwvzjrhvvgdgswlvzjjhjtdctlthlpzqhjvwwbpsclpgflcnsdshrqbhmczcwljqlndfnfrcdgmptpsltrcjccnpdchgnswdcpsslcslcjznzpgfhznhbgqhdqvddmqzdnmpshhdcjrsmfjllhfvjvmzzhzrvlbpzqngwmlwcmqnppqzncvjshfrpjlptvnqfrfcrfnbhwhpdqqvjhsqvsmprtgfrddwzjzlwhhqvjpfrwgwvwpszzsfzwjtwngdjfllhjrmqjtmvwsvggnswpqpjbtcrnhhhlzbrvhjdstnpctjlgsffrrbfdvjzhwsgthgfsqnvqdcjffsttlrjnhtqqdpfqpjtdgfwcdwzmwfvqgglsrmmqwbszclpzwldwcswpwfwldrfmmdndcptjbmnvgcpntqcdrcffvgnlpjmcqjpfmbmwjfpqzbzhqtqbzsghbnfvhphfzzhfznttpfrqwpmzjchpzzrdclhdltlqbjmjdfdjqlqbwptsghcnvtdscwgpqnlhhvsvglplhlrwpnzmdbsbrlhmpczzfz";
// start with window, 0, 0
// record the character
// move the window right
// check if the character was in the window
// if was in the window, then move window after the position it was seen
// in the window keep track only the smaller index
pub fn solve(signal: &str, window_size: usize) -> usize {

    let mut indices: HashMap<char, usize> = HashMap::new();
    let mut start: usize = 0;
    let mut end: usize = 0;

    for (index, byte) in signal.chars().enumerate() {

        match indices.get(&byte) {
            Some(recorded_index) => {
                let next_start = recorded_index + 1;
                for i in start..next_start {
                    indices.remove(&signal.chars().nth(i).unwrap());
                }
                start = next_start;
                indices.insert(byte, index);
            },
            None => {
                indices.insert(byte, index);
            }
        }

        end += 1;
        println!("Start: {}, end: {}, map: {}", start, end, indices.len());
        if indices.len() == window_size {
            println!("Found ya! {}", end);
            return end;
        }
    }

    return 0;
}

mod tests {
    use super::solve;

    #[test]
    fn sanity() {
        let signal = "abcd";
        assert_eq!(4, solve(signal, 4))
    }

    #[test]
    fn first() {
        let signal = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(5, solve(signal, 4))
    }

    #[test]
    fn second() {
        let signal = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(6, solve(signal, 4))
    }

    #[test]
    fn third() {
        let signal = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(19, solve(signal, 14))
    }

    #[test]
    fn fourth() {
        let signal = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(29, solve(signal, 14))
    }
}