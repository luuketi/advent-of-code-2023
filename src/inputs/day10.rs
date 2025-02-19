#![allow(dead_code)]

pub const INPUT1 : &str = "\
.....
.S-7.
.|.|.
.L-J.
.....";

pub const INPUT2 : &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

pub const INPUT3 : &str = "..77LJJ7|7.|F-L77.F77FLF-L-777L7.77.F7-77-F|-7-F-FL-LF7F77FFF-7F7F7F-7F7FFF777|.FL|.F|.FFF-7|FL777FF.FFF77F--L-77--|FJFFF7FLF-|-JF7J-F--7-77
-||L7L-7LJ7L7J|LJ.|.J7|L--7||7FFFJ..-F7.7F7JF--J7|J|||LJ|F-7L7LJLJ||FJF-7FJL77J-L-77F|F-J.LF77-||F-F--F|--777|7|||7JL7LJF7-JJFLFJF|J.|||L-7J
L|7L7F--J.|-JF-7J-77F7J-JFFJ-|-J|L-L7J|.L||L|J|.|L-77L-7||FJFJF7F-J||-L7LJF-JJ.LJ--7JL77LJ7LLJ-LJJL7-7J|J.|F-J--J.7|FJ.|||LJF7.J---7-J-|.|F7
.J7.JJJ-F|JF-7.|7-L|7JL7J7.FFLJF77-LL.L-FF7-F7-F7.FF7JFJ|||FJFJLJF-J|F7L7FJJ|..FJFL-.L|-J.|-7||JJJL|FL-JJF|.L-777LF-F7--.J..FJ|JL|-|7FFJ-F||
FF--|J.|.LF|F7-L---LFF-J-J-J-7LL|LFJ|7J-LL-.LJFF|-FJL7L7||LJFJF-7L-7LJL-JL-7J.F-FJL|J.LL7J.-.|.L|7.F|.|.FLJ-L.|L7J..LLLJ7|FFLF-.L-.||7L-F|L7
L77F|.L---JLJL|77LFJ|.F--LJF|J|F|-|LL7L7FLL7-FF--7L-7L7||L7FJLL7|JL|F----7FJJ-LFFJF7L7JLF77..7-J|LF-JFFLF.||JLJ.|.FJ.LL.L|F7JLF-77-|L7J7||F|
LLFJJ.JJ.J--7-L7|-|J|L7L-J-LJ7F-7||7|.-L-.LFF7L7FJF7|FJ|L-JL-7FJL7FJL---7|L--7.L--7J7|.F7J77..|7F-L7F-L-7FJ77.F7L-F7F7J.||LL7L7.FJFFF7---FLJ
7J.|-7F7.F7L|7LF||FJF-|7|.||7L--J7L-JJ.LL7.FJL-J|FJ||L7L--7F-J|F-JL7F---JL---JJJ|FF--7.||.7FF|-FLJLF--|-L7F|--L7|.LF||7--LJFJ7F---7J.|J|F77.
.7-F7-J|LJ7.LF-J|LFF77LJF-F-|JL7FJ.|J.|7-FFL---7|L7||FJ-F-J|JFJL-7F||F--7F7F7L||LFL-7|FJ|7LFF.|..-JJ|JL-J|FJJ.L|77..||7F|FFF|-L7L-J.J|-|LJL7
JL7JL-LJL|.7FJL|LJJ||L--LJLF7-.FJJ.|JFLJ.LF----J|FJLJ|F7L-7L-JF7FJFJ||F-J|LJL-77.F--J|L7|7-LF--7.|-F-7J.||LL-7.L|7-F||7--F7--J.||.|LJ-FJ---|
L-JJ|FJ.L-L-7|FJJ.-JFJ.LL.LLJJ-|J-7JFJJ77FL--7F7LJF-7LJ|F7|F--JLJ-L7LJL--JF7F-JJ7L--7|FJ|F7F|F7L--7L7|-.FJ7|7L-J|JFFJL7JF|LL7F-77|7|F7L7F|F|
|77.||.F.L|.|F|L-7J.J-7.|J.|JF-J-F7-7J|7|-F-7LJL7FJFJF7LJ|||F--7F7FJF--7F-JLJF7||-FFJLJFJ|L7LJ|F--JFJ|L|J|F7J.FFF7FJF-JFF77-L7LJ7LLLJ|7-7-|7
--|-7-----7FLFL7..-7.FFF7FFFJFJ|J||-F7-F-7L7|F7FLJ-L-JL-7LJLJF7|||L7L7|LJF7F7||-|7LL--7|-|FJ7-||F-7|FJ.FF-7.LFF-JLJFJ7F7|||LLL-J|.J7|F-.L-L7
L7LLF--J.FL7L|J.FF--F--J|7F|-|FF7||||L7L7|FJ|||F-7F7F--7|F-7FJLJ|L7|FJF7||||||L7F7-F--JL7||F7F|||FJ|L--7|FJ77|L---7L7-||||7.LL7LLL-L7.|-...|
LJ7-L7|7L|---L777|FFL-7FJ7F7LLFJ||L7L7L7||L7||||FJ||L-7|||FJL7F7L7LJL-JL7|||||FJ|L7L--7FJ|||L7|LJ|-|F--J||F7F7JF-7|FJFJLJL777.FF..FLJJJL-J-L
.J|FJFL|.||F|JF----7LF|L7-||77L7LJFJLL7LJL-JLJLJL-J|F-JLJ|L7FJ|L7L7F----J|||||L7L7L7F-JL7|||FJL-7L7||F7FJLJ||L7L7||L-JF--7L7JF-7.F-7J.7..|.|
7.F|L|.J.7JF|7L---7L-7L7L7|L7-L|F-JJ-FJF-7F7F-----7LJF-7FJFLJ.|FJFJL-7FF7|||||FJFJFJL--7|||||F7||FJ|LJ|L--7|L7|FJ|L7F-JF7L-JJ7LL-F-|-7|F-J7-
L77.F|J.-J-FJJJFF7L-7L-JFJL7L--JL7|.-|FJ7LJ||F7F--JF7|FJL---7FJ|FJF--JFJ||||LJ|7L7|F7F7|||LJ|||FJL-JF7L7F-J|FJLJFJ.|L--J|FLJJL-JF||J|LF7|LJJ
L7J7LJJ-|L-JJ|-FJL--JF-7|7FJF7F-7L77FLJ|F7JLJ||L--7|LJL7F-7FJL7LJFJF7LL7||||F-JF-J||LJ||||F-J|LJF7F-JL-JL-7|L7F-JFFJF--7L7J.7J.|FJ-J|7-L--JF
.L.|JLJJL7JJFJFL----7|FJL7L7|LJJL-JF----J|F7FJ|-F7LJ-F7|||LJF-JF-JFJ|F7||||||F7L-7|L-7||||L-7L7FJLJF-7F-7FJ|FJL7F7L7|F7L-J7.L7.LFJ.L||FJ.L7-
L7FJ.L|.|.F-|LF7|F-7LJL-7|FJL--7FF7L----7|||L7L-J|F7FJLJL7F7L-7|F7L7||LJLJLJ|||F7|L7|||||L7FJFJ|F--JFJL7|L7||F-J|L7|LJ|7F-7-J.|-7L77-F7F7-|J
LFLJ7L7-7JJFF7|L7L7|F-7FJ|L7F--JFJL7F7F-JLJL7L7F-J||L--7FJ||F7|||L7||L---7F-J||||L7L-JLJL-JL-JFJL7F-JF-J|FJLJL-7L7LJF-JFJFJ7|.77FJ||F|LFJ-|.
L7.|.FJJJL7FJLL7L-J|L7|L7L-JL7|JL7FJ||L----7L-J|F-J|F7J|L-J||||||FJ||F7F-JL7-|LJ|LL-----7F----JF7|L-7L-7|L-7F--JJ|F-JF7|FJLF77LLJ.L|JJ-|7-LL
7J77.|7--7LLJLFL7F7L-JL-JF7F7L7-FJ|L||F7|F7|F--JL-7|||FJF--J|||LJL7||||L--7L7|F-JF--7F--JL---7FJLJF-JF7||7FJ|F7F-JL-7||||F7||7F7|...L|-77JJ|
|7FJFFF7FFJJ.-LJLJL------JLJL7L7L7|FJLJ|FJLJ|F-7F7||||L7L7F7|LJF--J|||L7F7L7|||F7L7FJL--7F---JL7F7|F-JLJL-JFJ|||F---J|LJLJLJL7J-L7LL.L77|7.J
LL|---J|||-7..FF--------7F7F7L7L7||L-7FJL-7FJL7||LJLJL-JFJ||L-7L7F7|||FJ||.|LJLJ|FJL7F--J|F7LF7||LJL---7F--JFJLJ|F---JF7F-7F-JJLL-.J-7|L-J7|
|J|||.FLL77|FF7|F------7LJ|||FJFJ|L7FJ|F7FJ|.FJ|L----7F7L7|L-7L7LJ||LJ|FJ|FJF---JL7FJ|F-7LJL-J||L7F--7FJL--7L7F-J|F---JLJ|LJ.|JLFLJL|LJ.|-LJ
||LJ.-F-LLFF7||LJF7F-7FJF7|||L7|FJFJL7|||L7|FJFJF-7F7LJL7||F-J7L7FJL-7LJFJL7L7F7F7|L7LJLL-7F--J|FJL-7|L7F7FJ.|L-7|L7F-7F---77L7-J7J|||LFJL||
-77JFFJF|.FJLJ|F7||L7LJFJ|||L-J||FJF7|LJ|FJ|L7|LL7LJ|JF7|LJL-7F-JL7F-JF-JF7|FJ||||L7L7F7F-J|F7J||F7FJL-J|LJF7|F-J|FJ|FJ|F--J|-F7.J7FJ7.-J7|J
|.|L-7.FLFL7F7LJ||L7L--JFJLJF-7LJL7||L-7|L7|J|L-7L-7L-J||F--7|L-7FJL-7|JFJ|||-|||L7|FJ||L-7||L7|LJ||F--7L7FJ||L7FJL-JL-JL7|F7FJL7.F.|F.LLFJ|
|.7-JJ7|LFFJ|L-7|L7L7JF7L7F-J-L--7LJL7FJL-JL7|F-JF7L-7FJ||F7LJF7|L7F-JL7S7LJL7||L7LJL7||F7||L7LJF-J||F-JFJ|FJL7LJF------7L7|LJF-JF7-F|77LF7-
-FJFJ7LL7.L7|F7|L7L7L7||JLJF7F7F7L7F-JL-7F--J|L-7||F-JL-JLJL7FJLJFJL--7|.L7F-J||FJF--J|||||L7|F-JF7LJL-7||||F7|F7L-----7L-JL7FJJ.-LF-7FJ-LL7
FL7LF-JJL7-LJ||L7|FL7|||F-7|LJ|||FJ|JF7FJL7|FJF-J||L-7F-----J|F--JF7F7||F-JL7L||L7|JF7||||L7|||F7||F7-FJ|FJ|||||L------JF77FJ|J77-FJLFF7LL|F
L-JF7|JFF7J|FJ|J||F-J||||FJL-7|||L7L7||L-7|FJFJF-J|F-JL7F--7FJL-7FJ|||||L--7L7||FJL7||||||FJ||||||||L7|FJL7LJLJL7F--7FF7|L-JFJL-F-7J.LJJ..F7
LL7FF|-F||J-L7|FJ|L-7LJ|||F--JLJ||L7|||F7|||FJJL-7|L-7FJL7FJL-7FJL7|||||F7L|FJ||L7FJ||||||L7||||||||FJ||F-JF-7F-J|F7L7|LJF-7L-7J||J-JJFL-7LJ
|.J-||-FJ|FF7|LJFJF7L-7|||L----7L--JLJ||||LJ|F7F7|L7FJ|F7||F-7|L7FJ|||||||FJL7||FJL7||||||FJ|||||||||FJ||F-JJ||F-J|L-J|F-JFL--JFL|--|-L--7L7
-J.F|LJL7L-JLJF7L-JL--J|||JF7F7L-----7|||L7|||||||FJ|JLJ|||L7|L7||FJ|||||||F-J|||7|||||||||FJ|||||||||JLJL7F7LJ|F-J7F-JL-7FF---77J..|-L.L7JL
FLLJL.FLL7F-7FJL------7LJL-J||L------J|||FJFJ|||||L7L7F-J|L7||FJ|||FJ||LJ|||F7||L7FJ|LJ||||L7||||||||L7F--J||F-JL7F7|F---JFJF-7L--77|JFF.J..
|7.F.-|.FLJ-LJF7F7F--7L----7LJF7F---7JLJ||JL7|||||LL7||F7|FJ|LJF|||||||F-J|LJ|||FJL7L-7|||L7||||LJ||L-JL---JLJF--J|LJL--7FJFJLL7F-J77-7JFF-.
.FF-FJL-FF7F7FJ||||F7L----7L-7|LJF--JF77LJF-J|LJLJF7LJ||LJL7L--7||||FJ||F7L-7|||L-7|F7||||FJLJ||7FJ||F7F7F--7FJF-7|F---7||FJF-7LJF-7-7L-FL-F
FL|--7LFFJLJ|L7LJ|LJL-7F--JF7LJF7L---JL-7.L-7L----JL7FJL7F-JF7FJ||LJ|FJ||L7FJ|||F7||||||||L7F-JL7L7|FJLJLJF7LJ|L7LJL--7LJ||L|FJF-JFJ-|J.|-7|
J7LJJ|JLL7F7L7L-7L-7F-JL7F7||F-JL-7F-7F7L7F7|F--7F7FJL7FJL-7|LJFLJF-JL7LJ|||FJ|||||LJ|||||FJL-7FJF||L7F7F7|||F7FJF7F--JF-JL-JL7L-7L77F7-|7|F
L7.LF|||L||L7L7FJF7LJF7FJ||||L---7LJJLJL-J|LJL-7LJ||F-JL7F-JL----7L7F7L-7FJ|L7|LJ|L7FJ|LJLJF--JL7FJ|FJ|LJLJL-JLJFJ|L7F7|F-----JF7|FJF-7JL7F|
LF7F7|F7|LJFL7LJFJL7FJ|L-JLJL---7L-7F7F7F-JF7F7L-7LJ|F7FJL--7F7F7|J||L7FJ|FJFJL-7|FJL-JF---JF-7FJL-J|FJF------7FJLL-J|LJL7F--7-|LJL-JFJ--J|J
-J|JLFFF----7L-7|F-J|FL--------7|F7LJLJ|L7FJ|||F-JF7LJ||FF--J||||L7|L7||FJL7L7F7|||F---JF--7|FJL-7F-J||L-----7|L-7F7L|F--J|F-JFJF7F--JJFLJJ7
LLL-FFFL---7L--J|L--JF--7F-----J|||F---J7LJFJ||L--JL-7|L7L7F7||||FJ|FJ|||F-J-|||||||F--7|F-J|L-7FJL--JF---7F-JL--J|L7|L---J|F7|FJLJF7.|77J.|
L|L7.||LF--JF---JF--7L-7|L-----7LJ|L--7F7F7L-JL7F7F7FJL7|||||||||L7|L7LJ|L7F-J|||LJLJF7LJL7FJF-J|F7F-7L--7LJF7F---JFJ|F----J|LJ|LF-JL7F7|JFL
||F7FLJJL--7L7F--JF7L--JL-7F7F-JF7L--7LJLJ|F-7FJ|LJ||F7|L7LJ|||||FJL-JF-JFJL-7|LJF7F-J|F7FJ|JL--J||L7L7F-JF-JLJF---JFJL-7F-7L7FJFJF-7LJL77|.
FJFJJ7L.J7LL7||F7FJL-----7LJLJF-JL--7|F7F7LJFJL7L-7|LJ|L7|F-J||||L---7|F7||F-J|F7||L-7LJLJFJF-7F7||LL7|L--JF7F7L----JF-7LJFJ-||-|FJ-L---JJFF
F--JLF-FLFJFLJLJLJF------JF-7FJFF---JLJ|||F7L-7|F7||F7L7|||F-JLJ|F-7FJLJ|L7L-7||LJL--JF-7FJLL7LJLJ|F7|L----JLJ|F-----JL|F-JF-JL-JL-------7-L
J|FJFL.7J|7F----7FJF7F7F7FJ.LJF7L-----7LJ||L-7||||||||FJLJLJF---JL7|L--7L7|F7LJ|F----7|F||F--JF--7||||F7F7F7F7LJF-7F--7|L--JF7F-7F--7F---J7.
FLJ.JJ.JFF-JF--7|L-JLJLJLJF77FJ|F----7L-7|L-7LJLJLJLJ|L-7F7FJF7F7FJ|F--JFJ||L-7||F7F-JL7||L7F7|F-JLJ|LJLJLJLJL-7L7LJF-JL-7F7|LJFJL-7|L--7L7J
77.L.L-7FL--JF7||F7F--7F--JL-JFJL---7|F-JL--J.F7F7-F-JF7||LJFJ|||L7||JF7L7||F7LJ|||L7F7||L7LJLJL---7|F7F----7F7L-JF7L---7||LJF7L-7FJ|F7FJ-L.
LLLJ|FLL.LF7FJLJLJLJF7LJF-7F-7L-----JLJF------JLJL7L--J|||F-J|||L7||L-JL7||LJL7FJ|L-J||||FJF7F-----JLJ|L---7||L--7||F7F7||L--JL7.|||LJLJ|-|7
F7-7JF-|FF||L-----7FJL-7|JLJ||F--7F7F7F|F-----7F--J7F7LLJ|L-7FJ|F||L-7F-JLJF-7LJJL7F7|LJLJ7||L-7F7F--7L----JLJF7-LJLJLJLJL-7F7FJFJL7J.|.F.77
F--J--77FFJL------J||F-J|F---J|F7LJLJL-JL-7LF7LJ.F--JL--7|F-J|FJFJL-7||F7F7L7L-7F7LJLJF7F--JL-7||LJ.FJF-------JL------7-F77LJLJ-L--J-FF||F||
L|7FLLL77L-7F-----7|FJF-JL---7||L--------7L-JL---JF--7F-J||FFJL7L7F7||LJLJL7|F7LJL----J|L----7|LJF--JFJF------7F------JFJL--77F---7JL7|.||L-
--FLJ|FLL7.LJ-F7F-J|L7|.F----J|L7F------7|F7F7F7F7L-7|L-7|L7L7FJJLJ||L7F-7FJ||L7F-----7L-7F--J|F7L---J|L-----7|L--7F---JF-7FJFJF--JF7J-F.JJ|
.F7|.||L7F7F--JLJF7||LJFJF--7FJFJ|F---7FJ||LJLJLJL--JL7FJL7L7LJJ||F|L7||FJ|JLJ7||F7F--JF-JL7F7LJL--7F7F7F--7FJL7F7LJF-7FJLLJFJFJLF--7JJLFJ||
FJ|FF.|.F|LJF7F7FJ||F--JFJF-J|-L7|L-7LLJ-|L7JF--7|F7F7LJF7|FJ|7JFJFJFJ||L7L7F--J|||L--7|LF7||L----7|||||L-7||F7LJ|F-J-||F77|L7|F-JF-J7.7|.FJ
7.|L|-|--|F7|||LJ|LJL---JLL-7|F-J|F-JF7F7L-JFJF7L-JLJL--J|LJ.|J-7LL-JFJL7L7||F7FJ|L---JL-J|LJF----JLJLJL--JLJ|L-7|L7F7LJ|L7F7|LJF-JF77-LF7JJ
|.|.||L--LJLJLJF7F7|F------7LJL--JL7FJLJL---JFJ|F7F7F---7|.|7J|F--J.L|F-JJ||LJLJ7L--7F7F-7|F7|F-7F7F-7F7F---7|F-JL-J||LFJFJ||L7FJF-JL7|-|||7
LJ||J-.J7L..LJF|LJL-JF--7F7L7F7F7F7LJF7F--7F-J-||||||F7||L-777FJ-|..FLJJJ.LJ-F------J|LJFJLJLJL7||||FJ|||F-7LJL---7FJL7L7L7||FJL-JF-7L7FJL-7
|.7J.F|FJJ-F7JFJF7F-7|F7LJL7LJLJLJL-7||L-7LJJF7LJ|||LJL7L--JJF7.77JF7|FL--JJ.L7F7F7F7|F-JF7F-7FJLJLJL7|||L7L------J|F-JJL7||LJF7F-J|L7LJF-7|
|7L-F-JFL|.||FJFJ||FJ||L7LFJF----7F7LJL-7L--7|L7JLJL--7|FL7L7|77L-.FFJ--J|LJ-L||LJLJ||L--J||.LJF-----J|LJFJF----7F7|L-7F7|||F-J|L--7FJF7L7LJ
||F|J.L|--FJ|L7|FJ|L-JL7L-JFJF---J|L-7F7L---J|FJF----7LJ7|F-||L|L|.FJ-LLF-7|.L||F---J|F---J|F-7L------JF-JFL7F--J|||F-J|||||L-7L---JL-JL-J7|
FJF7.-.L||L7|JLJL-JJF7|L7F7L7L7F--JF7LJL7FF7J|L-JF--7||F7F7FLJFL--JJ-FJ.F7-|7.LJL---7|L-7F-JL7|JF7F7F7JL----JL7F7|||L-7|||||F7L7F7F7|F--77L7
J.|F-|-FFLFJL---7|F7||F7|||FJFJ|F--JL--7L-JL7|F-7L-7|L7||F-7--LJJFJ-F|.FJJ|.-7.L-F--JL-7LJF--JL-J||||L--7F---7LJLJLJF7LJLJLJ|L7LJLJL7|F-J-FF
J.LJFF.F|LL-7F-7L-JLJLJLJ|LJ7L-JL--7-F7L----J|L7L--JL-J|LJFJ|F-7FJ||.--L7.LJF|7-FJF-7F-JF7L-7F7F7LJ|L--7LJF-7L7F-7F-JL7F-7F7L7L--7F7LJL-7-J|
FJFFLJF7|.LLLJFJF---7F7F7L--7LF7F7|L-JL------JFJ7F-7F-7|F-J7L7LLL-|7FJ|FJ77L|L--L-J.LJLFJL--J|LJL-7L---JF-JFJFJL7|L--7|L7||L7L--7||L-7F-JF||
L-F|F--|-J.J.FL-JF7FJ|LJL---JFJLJL----7F7F--7FJF7|FJL7LJL7L-J7--J-L77JLJJ|F7|7JJF-7F7F7L-----JF7F-JF7F-7|F-J7L7FJL-7FJ|FJ|L7|F--JLJF7LJF7-L|
LF||LJ.7.L|FFF-7FJLJFJJF--7F7L-------7||LJF7|L7|LJL7J|F7FJ7|.|F7J.FF7.||FJ||J|JFJFJ|LJ|F7F7F--JLJF-JLJJLJL---7|L-7FLJ|LJFJFJ|L-----JL--J|JLJ
LL-7|LFL--|7LL7|L---JF7L-7LJL-7F-----J|L--J|L7LJF-7L7||LJJ|-7|L7F|7.L-7-FJ||-|-L7L7L-7|||||L-----JF7F7LF7F---JL--JF---7FJFJFJF7F-7F7F7F-J|FJ
7.FJ77.|FL7-F-J|F----JL--JF7F7LJF----7L7F7FJ-L7FJLL7LJL-7.|JFJ-LJJ..J7L77|F||JF7L7L--JLJLJL-------JLJL-JLJF--7F--7L7F-JL-JL|FJ||FLJLJ||J|F-|
.|7LL-F-F7JL|F7LJF---7F---JLJL--JF---JFLJLJFF-JL7F-JF7F-JFL.LJFF7|F|-|L77FF7F-JL-JF-7F7F7F7F--------------JF7LJF7|FJ||F---7|L7|L--7J.LJ|FJFF
FL7-|..FJ|FLLJL-7|F-7LJF---7JF7F7L---------7L-7FJL-7||L-7F|7|JF7JJJLF77L7|||L-7F--JLLJLJLJLJF7F7F7F--------JL--JLJ|FJFJF7FJL-JL-7FJL-7FLJLLJ
FL.|L|.L.FF-----J|L7|F7L--7|FJLJL----------JF7LJ-F7LJL-7L--7|.LJ.|JF||77|FJL7LLJF-7F----7F--JLJ|||L7F7F7F----7F7.FJL-JFJ|L-----7||L-J-7|7F.|
LL-7F7-|FFL7F----JFJ||L---JLJF7F-7F7F7F---7FJL7F-JL---7L--7|F77JFF--J|F77|F-JLF-JFJL-7F-JL----7LJL7||LJLJF--7LJL-JF---JFL------J|L7J7FJ-|L-7
||LFFJ7|--LLJF7-F-JFJL---7F-7||L7LJLJLJF--J|F-JL-7F---JF7FLJ||F77L-7FJ||FJL--7L-7|F7FJ|F7F7F7.L--7LJ|F---JF7L-7F-7L--7F7F-7F--77L-JL7-JFF7L|
LJ||.LL.7JJFFJL-JF7L--7F-JL7LJL-JF7F7F-JF7J|L----JL----JL7F-JLJ|F7|||FJ|L7F--JF7||||L7LJLJLJL--7FJF-JL-7F-JL-7|L7L7F7LJLJFJ|F-JF-7J.||J|LJJJ
FL7F-7L..LFFJF--7|L7F7|L7F-JF7F--JLJLJF-JL7|F---------7F-JL---7||L7|LJFJL||-F-JLJLJL-JF7F----7FJ|FJ.F-7LJF---J|FJJLJ|F7F7L-JL--JFJJF-|-L-J|J
|-F-FLL7--FL-JF-J|.|||L7LJF-J|L------7|F--J||F7F--7JF7LJ.JJLF7||L7|L-7L--JL7L7F-7F7F7FJLJF---J|FJ|F7L7|F7L---7|L7JF7LJLJL--7F-7FJ.LF---7|.F7
L7JL--J77JL.F7L--JFJ|L7|F7L-7|F-7F7F-J|L--7|LJLJF7L-JL-7J-F-JLJL-JL-7L--7F-JJLJFJ|LJLJLF-JF7F7LJFJ|L-JLJL-7F-JL-JFJL-----7FLJF||.F-J7J.F7--L
JJLF|.J.7.LFJL7F-7L-JFJLJ|F-J|L7LJLJF7|F--J|F---JL-----J|FL------7F7L7|FJL-7F-7L-JF----JF7|LJ|F7L7L7F7F--7LJF7|F7|F---7F7L---7|L77F.F7|L||.|
|7.|-FJFLF7L-7||FJF7FJF-7LJF7L-JF7F-JLJL7-FJL--7|F7F7F7F7FF7F----J|L7L-JF-7LJFJF-7L---7FJ||F-J||FJ|||LJF-J.FJL7|||L-7LLJL----JL7|7J7LL7-F|-L
||.|LJ.-J|||FJLJL-J|L-JFJF-JL-7FJLJF----JFJF7F7L-JLJLJLJL7||L-7F--JLL--7|FL--J-L7|FF7LLJ|LJ|F7|||F-JL-7L---JF-J|||F-JF7F------7LJ|.F7L-JLJ7|
|7-J7.F.L|L-JF7F7F7L-7-L-JF---JL7F7L7F-7FJFJ|||F7F----7F-J||-|LJF7F----JL7F7F7F7||FJL77F--7LJ||||L-7F-JF----JF7|LJL7FJLJF7F--7L---7-JJ--JF77
F-7||F77FL-7FJLJLJ|F7L-7F-JF-7F7||L-J|FJ|FJ-LJLJ|L---7LJF7||F7F7||L-----7LJ|||||||L7FJFJF7L7FJ|LJF7LJF-JF7LF7|||F-7LJF--J|L-7|F--7|JFL7.FF|.
L--|.F-J.LL||-F---J|L-7LJF7|F||||L---JL-JL-7|F7-L---7L7FJLJ||||LJ|F-7F-7|F-J|||||L7|L7L-J|FJL7L-7|L--JF7|L-JLJLJL7|F7L--7L--JLJF7LJ7F-F77J.-
J|J|77.LL7LLJFL-7F7|F-JF7|||FJ|LJF7F-7F-7F-JFJL----7L-JL-7FJ||L-7LJFJL7LJL-7|LJ|L7LJFJF7FJL-7L--JL----JLJF-7F-7F-JLJL7F-JF7F7F7|L-7LJJ|L7F-J
F-F-77-7J|L|||FFLJLJL-7|||||L-JJFJLJ-LJFJ|F-JF-----J.F7-FJL7||JFJF-JF7L---7||F-JF|F7L-J|L7F-JF-7F---7F7FFJFJ|FJL-77F7LJF7|||||LJF7L7JF7LJJ7|
7JLJ7-FLFJ.F|JLF------J|LJLJ7F--JF----7L7LJF7L----7|FJL7L-7LJL7L7L7J|L-7F-JLJL--7LJL7F7L-JL7||FJL--7LJL7L7L-JL7F7L-JL--JLJLJLJF-JL-J.J7F|-F7
|--|J-L-JJ-|J|FL--7F-7FJF----JF7FJF--7L7L-7||F7F7FJFJF-JF7|F--J7L7|FJF7|L--7F---JF7-LJL--7FJFJL7F--JF-7L7L--7FJ||F7F---7F--7F7L-7J7L|L|7|L||
|.LJJ.L7|-F|.-LF--J|FJ|FJF7F7FJLJ7L-7L-JF7LJLJLJLJFJFJF7||||F7F--JLJFJLJF--JL--7FJ|F-7F7FJL7L7FJL-7FJLL7|F7FJL-JLJLJF7L|L-7LJL7FJFFF|.LFL-||
L-7F-7FL-7FJ||.L7F-JL7||FJLJLJFF----JF7FJL-7F7F--7L7L-JLJ||LJ|L----7L7F7L--7F--JL7|L7|||L-7L7|L7F-J|F--J|||L--------JL7L--JF-7|L77L7L|7L-L|7
L7F-J|L-LL7JF--7LJF-7LJLJF7F7F7L7F---JLJF7FJ|||F-JFL--7F-J|F7L7F7F7|FJ||F7F|L7F77||FJ||L7FL7||FJL-7|L7F7LJL7F7F---7F-7L----JFJL7|7||JL|L7L77
LL77.|.F7F|7|F7L--JFJF--7|LJLJL-J|F-----JLJ.|||L-7F7F7|L7JLJL7LJ||||L-JLJ|FJFJ|L7||L7||FJF7|||L-77LJ-LJ|F7FLJLJ.F7LJ7L--7F-7|F7LJ--7L7.FF7.|
FLJJF-7.L7FFLJ|F7F7L7|F-JL-7F-7F7|L-----7F7FJ||F-J||||L7L77F7L-7||||F----JL7L7|FJ||FJLJ|-|LJLJF-JF-7F7FLJL------JL7F---7LJ||LJL-7.-|JF-FJL77
F7FLF7|.J.|LL|LJLJL7LJ|F7F-J|JLJLJF-----J|||FJ||F7|||L7|FJFJL7FJLJLJL7F7LF7L7|||FJ|L7F7L7L-7F7|F7|FJ|L-------7F7F7||F--JF--JF7F-J7-J7L-|7L||
L77JJL--J-L7FF7F7F7L-7LJLJF7|F---7L7F7F77|||L7|LJ|||L7|||FJF-JL---7F7LJL7|L7||||L7|.LJL7L-7|||LJ||L7L-------7LJ|||LJL--7L-7FJ|L-7LFJJ7.L--JJ
|L7--J.|.|FF-J||LJ|.FJF-7FJLJL--7||LJ||L7|LJFJL7FJ||F||||L7|F7F--7LJL-7FJL7LJLJ|-|L7F-7|F-JLJ|F-J|FJF7FF----JF7LJ|F7F--JF7|L7|F7|L-JL-J-|7JL
F-JF--F.FF-JF7LJF7L7L7|FJ|F7F---J|F--J|FJL-7L-7||FJL7|LJL7||||L-7|LF7F|L7FJF---JFJFJL7||L7JF-J|F7|L7|L7L-----JL-7LJ|L---J|L-JLJLJ-J77.L----|
J7|7|7|7|L7FJ|F7|L7L-J|L-J||L---7|L7F-JL7LFJF-J|||F7|L--7LJ||L7FJ|FJ|FJFJ|FJ.F7FJFJF7||L7L7L7FJ|||FJL7L-7F---7F7L-7L-----JF7F7J-|-JLJ7.|LLF|
LFF-77-F-JLJFLJLJJL-7FJF--JL----JL7|L7F-JFJFJLFJ|LJ|L-77L-7||FJL7||FJL7|FJ|F7|||FJFJLJL7|FJFJ|FJ|||F7L-7||F7.LJL-7L-----7FJLJ|-F|-F7--7-.FLJ
7LJFLJLFJ.|.F-LLF---JL7L7F-7F-7F-7LJF||F7L7|F7L7|F7L7FJF7.||||F7||||F7|||FJ||||||-L7F--J||F|FJL7|||||F7||LJ|F7F7.L7F-7F-J|F-7|7-77||L|L7F-J.
L7LJFF7.F||F.-.LL----7L7||FJL7LJ7L7F7|LJL7|||L7|LJ|FJL7||FJLJ||||||||LJ||L7|||LJ|F7||F7FJL7|L7FJ|||||||||F7LJLJL-7LJ-LJ|FJL7|L7F|77L7JFJJ-LJ
.|7FFFJ-J|FF--F-F7|F7|FJLJL7FJF---J||L-7FJ|||FJL-7||F-J||||F7|||||||L7F||FJ|||F-J|||||LJF-JL7||FJ|||||||LJ|F7F7F-JF7F---JF7|L-JF7F7-..|J-FJJ
F--7FJF-J-F-.-FFJL-J|LJF7F7||FJF--7||F-J|FJ||L7F-JLJL7FJ||FJ|LJ||||L7L7||L7|LJL7FJ|||L7FJF7FJ||L7||||||L-7LJLJLJF7|||F---J|L--7||||7F|..-|-7
.FF-L-7-F-LL-LJL---7L--JLJLJ|L7|F-J|||F-JL7LJFJL----7||FJLJFJF-J|||LL7||L7||LF7|L7|||FJL7||L7||JLJLJ||L-7L-7F7F7|LJLJL--7L|F-7LJLJL7LL7..JJ.
F-LL|FJ.-.|.LLJF7F-JF-7F7F7FJFJ||F7|||L7F-JF7|F--7F-J|||F7FJFL-7||L-7|||FJLJFJLJFJ|||L-7|||FJLJF----JL-7L7FJ|LJLJF-----7L7|L7|F7F--J-FJJ.FLF
-J|7F-L-7.LJ7|-|LJF7|FJ||||L7L-JLJLJLJFLJ7FJLJL7L|L7|LJ||||LF7FJ||F7||||L-7JL-7FJ7LJ|F-J|||L--7L--7F7F7|FJL-JF7F7L----7|FJ|FJLJLJJ7.-7.FFF7|
LFF7JFL-|7|-|.-L-7|||L-J|||FJ-F-----7F7F--JF7F7L7L7L7F-J|||FJ|L7|LJ||LJ|F7|F--JL---7|L7F||L-7FJJF-J|||LJL7F7FJLJL--7F7||L7|L----77F---7FF-77
.7LL-FJF-77FF-.F|LJ||.F-J|||F7L----7LJLJF--J||L-J-L7|L7FJLJL7L-JL-7|L7FJ|||L-7F-7F7|L7L7||F-J|F7L-7||L--7LJ|L---77FJ||||FJL--7F7|FJJ.LLJ7FFJ
F-FJ--F7.LJF|7L-FF-J|FJF7||||L7F7F7|F--7L-7FJL----7||FJL-7F-JF7F-7||FJL7|LJF-JL7|||L7|FJ||L-7||L--J|L--7|.FJF7F7L7L-JLJLJF7F7LJLJJJF7LFL-LJJ
|-||LLJJFFLJJ7.F-JF7|L7|||LJL7LJLJ||L-7L7FJL7F---7|||L7F7||F-J||-|||L7.||FFJF7FJLJ|FJ||.LJLFJ|L7F-7|FF7LJFJFJLJ|FJF------JLJL--7JJ.FF7JFFJF7
J.-J7-F-FFJ7F|.L7FJ||F||||F--JF7F7LJF7L7||F-JL--7||LJL||LJ|L77||FJ|L7L7|L7|FJ|L7F-JL7|L7|F7L7L7||FJL7|L-7L7L-7FJL-JF7F-7F7F7F-7L7-J.|LF-JJ.|
LF-||77.L7L-.|7-LJFJ|FJ|LJ|F--JLJL7FJ|FJ||L7F7F-JLJF-7|L-7|FJFJ|L7|FJFJL7|LJ7|FJ|F7FJ|FJFJL-JFJLJ|F7LJF7|FJF7|L-7F-JLJ.|||||L7|FJ7J.F-JJ|.FF
--.-77FJJ|-7.---J7L-J|FJF-J|F-----J|FJ|F||-LJ|L----JFJL-7|||LL7L7|||FJF-JL--7||-LJ||FJL7L---7|F-7LJL7FJLJL7|||7FJL--7F7LJ||L-JLJ.FFLF77F|77J
L|J.L|FJLL7LJ.L-LF---J|FJF-J|F7F7F7|L7L7LJF7FJF7F--7L--7||LJF-JFJ|||L7L-7F--J|L-7FJ||F7|F--7|LJFJF--JL---7|||L7|F7F7LJL-7LJ-F7JJFFF.|L7JL-J|
.|F|JL|J|.J...|--L7F7FJL-JF7LJLJ||LJ7L7L--J|L7||L7FJF7FJ||F7L-7|FJ|L-JF7|L-7L|F-J|FJ||LJ|F7LJF7|FL7F7F7F7|||L7|||||L7F-7L---J|F-J-L7|--7|JLL
.|-|.LJ7L-J7FF-F.LLJ||F---JL7F7FJL-7F-JF7F-JFJ||FJ|FJ|||LJ|L--J|L7L-7F|LJF7L7|L7FJ|FJL7FLJL-7||L-7||LJ||LJLJF||LJ||||L7|F-7F-JJ.|J--.|FF-7J7
.|7.|F|||7-7FF-L.|FL|||F---7LJLJF7FJ|F7||L-7L7||L7||FJL--7|F7F7|||F7L7|F7|L-JL-JL7||F-JF----J|L7FJ||F-JL----7||F-JL7|FJ|L7||J|J7J-FL7-F|J|F|
7L7J|L7LJ-FLJJ.|FFF-LJLJF7FJF7F7|||FJ|LJ|F-J-|||FJ||L7F--J||||LJFJ||FJLJ||F7F7F-7LJ|L7FJF7F7FJL||FJ||F7F7F-7|||L7F7||||L-J||||F|...FF--L-FFL
|-F-J.F7J-|7|--J7JJJLF--JLJFJLJ|||||FJF-JL--7LJ||||L7|L-7FJ||L7FJFJ|L-7.|LJLJ|L7L7FJFJL7||||L7FJ|L7|LJ|||L7LJ||LLJLJ|L---7LJ7-JJJFF-|JJ|FFJ|
|.77L-J|-FJF77LLJFF.LL-7F-7L7JFJ|||LJFJF7F-7|F7|L7|FJ|F7|L7||FJL7L7L7FJFJF--7L-JFJ|FJF7LJLJ|FJL7L7|L7FJ||FJF-JL----7L7F7FJJJ|.L|7L..|||.||L7
||L7-|LLLL7LL|7||F|F|.LLJFJFJFJFJLJF-JFJ|L7||||L7|||FJ|LJFJ|LJF-JFJ7LJ.L7|F7L7F7L7||FJL----JL7FJFJ|FJL7|LJ7L--7F7F7|FJ|LJJJFL|.||.FF-JL-7J-7
7L|L-J7.F-J7.L7-LJL-77-J-L-J||FJF--JF7|L|FJ|LJ|FJ|||L7L-7L7L7|L7FJLF7F7FJLJL7|||FJLJL--7F7F-7|L7L7||LF|L-7F---J|LJLJ|FJ.|LLJFJFJJF|-7|-LJ.F|
F--J77||JF.FF-J.LFJ..F7FF7F--JL7L-7FJ|L7|L7L7FJL-J|L7|F7L7|FJF-J|F-JLJLJF-7FJLJ|L--7F--J|||FJ|7|FJLJF-JF7|L---7L7J|7LJJ7|J|L--L7FJL-LJ7JL7LJ
F-J-FF||F--7.|F-7J77.LF-J|L7F7FJ7FJ|7|FJL-JFJL--7||FJ|||FJ||LL--JL--7F-7|J|L7F7L7F7|L--7|LJ|FJFJL--7L7FJLJJ.FFJFJF--J.LL|.7.JFL||FJ|.LJLLL-F
.F-LF..F7J7|7|FLJ-FJ-7L-7L-J|||F-JFJFJL7F--JF7F7L7LJFLJLJL||J|-F7F7.||FJ|FJFJ||LLJ||F--J|F-J|FJF7F7||LJF7|-FFL7L-7.-|7|F|7L7FF||7JFJ7F|J7.|F
-7-.LLF7J.F-.FJ-J|J-LJ.LL--7||||F7L7|F7|L--7|LJL7|JLJ.LF--J|-|FJLJL-J||FJL7L7|L-7JLJL-7FJ|F-J|FJ|||L---J|7J|L-L--J7|LFJ-J|.L-F7J..LJFLJ77.-J
J|.L.JLLFF.LL7.|.-77LF7JLLLLJ|||||FJ|||L7F-JL--7LJ7.L77|F7FJ.LL7F7F7FJ|L7JL7||F-JL|JF-JL7LJJFJ|J|||F7F7FJJL-|FJ|LLLL-7J||F-L7.L|F7LLFLLJ7L.|
.-J77J||.|77.|7||LJ-.JJ7.F||7LJ||||F||L-JL7F---JJ.J7J-L||LJJFL|LJLJ|L7|FJ|FJLJL---7FL--7|J.LL7|JLJLJ||||7.77||-JJFJJ-L||F7JF|7F7F|.FF7|.|JJJ
.||F|L-JFLF--L7--.|.|J|.F-7JJ|LLJLJFJ|F7.FJL---7J-|LJJ|LJFJLF-LJJ|LL-J||F7L7F7F7F-J7LJFLJ.|.|LJ7J-J.||LJJ7.F-.7|.LL.|F-.L-.|.|LF|JF|||F-|J7.
LF--J-JF7-|.F-JFL-L|J.F77.LJ.J.FJ.FL7||L-JF7F7FJ7..|.L||LJ7.|J||LJLL|J|||L-J||||||L77L7F|.F-7.JJFF|FLJ.|J-FJ7|-LJ-F-7JF7..LJ.-7|L7LFLL7J|L|.
|LFJ-LLF|F|JJ.FLJLL-7.LF|F|.||-J7LF-J|L--7|LJLJ.|7..-F--7LL-|-FL-JFJ||LJ|F7FJ||LJ--LL---7||L.J.7-L7||JLJ.L7.FLJL77FLJLFL7-L-.L|7-|L77-|-|.|7
7.|J7|.L|LJF77F7L-J|7.L7|F|--|7L|LL--J|F-JL--7JF|----|J.FJ|FF-J|F-|-7FLJLJ|L7LJ7-J||F-7.-7-|7||L7FLLL7|JF-J-J..F|7|LFJ.|L-F---.7L|||L-|.LL-7
F-L7LF7.|FFJ|L.FJL7LJ-7J--F77L7F||L|J--L7F7F-J7|||-LJ.FLJ-F|.FFJL7.FLF7JJFL-JL|F77F77-FJ||.FJ-.F-|7LLLJ.FJ|.-7-F|-F-|JF77F-7|L|J--LJJ-|F|L-L
L|JL7F-L-7J7|..L7LFFJJJL7LF77L7JLJ-.L-LFJ||L--7F-L7.F|FL.-LJ--.J7L.J--J7LF-JJF-7J7F777J.F7FL7.F-7L-7.|77L-F|F--7J.J-7-L--||L-.L-JLJJ..L-L--|
|..LL-7J.-.LLF.L|.FJJ...J..L7.LJJLJLJ.LL-JL---J-J..F-L7LFJJJ-|.LJLJJLLLJLLJJLLJLLL-L--J.L|J---|L|JJL7LLL-7-J-|-|J.L-J.J.J.LJ-7J.LL7J..LLJJJ.";