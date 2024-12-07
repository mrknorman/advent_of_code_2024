use std::collections::HashMap;

fn rules() -> HashMap<i8, Vec<i8>> {
	let ruleset = r#"24|55
38|32
38|21
48|51
48|92
48|14
78|35
78|54
78|87
78|44
72|29
72|65
72|87
72|82
72|37
23|56
23|51
23|81
23|83
23|25
23|87
25|14
25|18
25|55
25|22
25|15
25|77
25|17
54|96
54|65
54|33
54|21
54|36
54|29
54|56
54|89
56|78
56|88
56|77
56|52
56|22
56|81
56|14
56|85
56|17
33|23
33|22
33|83
33|14
33|89
33|25
33|55
33|92
33|72
33|78
96|78
96|21
96|14
96|77
96|92
96|68
96|88
96|24
96|83
96|55
96|91
97|56
97|54
97|17
97|43
97|13
97|33
97|48
97|96
97|65
97|89
97|83
97|37
35|92
35|23
35|25
35|52
35|24
35|33
35|51
35|21
35|55
35|88
35|17
35|56
35|89
52|44
52|61
52|88
52|82
52|64
52|71
52|51
52|99
52|97
52|68
52|85
52|91
52|18
52|55
22|43
22|61
22|72
22|68
22|15
22|87
22|64
22|85
22|97
22|88
22|51
22|71
22|18
22|63
22|91
29|14
29|81
29|35
29|89
29|96
29|21
29|13
29|36
29|23
29|65
29|48
29|17
29|24
29|52
29|33
29|99
65|17
65|15
65|25
65|48
65|56
65|22
65|52
65|21
65|92
65|23
65|36
65|83
65|24
65|55
65|37
65|89
65|81
36|77
36|96
36|83
36|99
36|56
36|89
36|68
36|21
36|55
36|92
36|78
36|88
36|51
36|72
36|52
36|14
36|24
36|17
32|24
32|83
32|13
32|29
32|21
32|25
32|92
32|56
32|33
32|99
32|96
32|14
32|35
32|22
32|88
32|17
32|65
32|81
32|37
64|42
64|48
64|37
64|96
64|44
64|56
64|71
64|76
64|36
64|29
64|33
64|23
64|65
64|13
64|25
64|38
64|43
64|32
64|97
64|54
55|64
55|76
55|61
55|43
55|38
55|87
55|71
55|72
55|54
55|68
55|18
55|67
55|85
55|63
55|51
55|78
55|77
55|44
55|15
55|97
55|91
43|14
43|96
43|32
43|89
43|48
43|35
43|23
43|36
43|83
43|24
43|33
43|52
43|56
43|65
43|13
43|25
43|21
43|17
43|54
43|76
43|42
43|37
92|85
92|71
92|38
92|77
92|64
92|76
92|44
92|51
92|63
92|68
92|87
92|55
92|88
92|18
92|99
92|15
92|72
92|97
92|43
92|82
92|61
92|91
92|78
17|18
17|81
17|82
17|52
17|92
17|77
17|64
17|21
17|14
17|68
17|51
17|83
17|99
17|24
17|91
17|72
17|88
17|15
17|87
17|85
17|55
17|22
17|67
17|78
83|68
83|77
83|18
83|99
83|67
83|51
83|15
83|72
83|92
83|22
83|82
83|85
83|88
83|63
83|78
83|44
83|52
83|24
83|91
83|55
83|87
83|61
83|14
83|64
76|25
76|14
76|56
76|22
76|89
76|23
76|42
76|35
76|83
76|29
76|48
76|32
76|81
76|33
76|96
76|13
76|65
76|21
76|36
76|52
76|24
76|17
76|37
76|54
67|32
67|71
67|33
67|25
67|56
67|97
67|42
67|43
67|38
67|37
67|64
67|29
67|76
67|35
67|48
67|44
67|36
67|82
67|54
67|23
67|61
67|63
67|65
67|13
71|25
71|48
71|29
71|33
71|96
71|42
71|17
71|89
71|97
71|35
71|37
71|54
71|23
71|43
71|83
71|36
71|21
71|13
71|38
71|76
71|81
71|65
71|56
71|32
42|22
42|35
42|29
42|24
42|92
42|36
42|32
42|99
42|89
42|14
42|48
42|65
42|56
42|17
42|37
42|96
42|23
42|25
42|13
42|21
42|52
42|81
42|33
42|83
51|67
51|78
51|61
51|71
51|63
51|97
51|42
51|87
51|72
51|77
51|18
51|29
51|15
51|64
51|85
51|82
51|44
51|43
51|68
51|54
51|38
51|76
51|91
51|32
85|61
85|67
85|76
85|25
85|37
85|33
85|36
85|32
85|23
85|97
85|43
85|63
85|38
85|35
85|13
85|48
85|29
85|44
85|64
85|82
85|65
85|42
85|54
85|71
81|82
81|72
81|52
81|87
81|91
81|15
81|14
81|83
81|92
81|22
81|78
81|68
81|77
81|18
81|24
81|67
81|51
81|85
81|64
81|99
81|88
81|55
81|61
81|21
63|29
63|44
63|38
63|89
63|36
63|48
63|25
63|33
63|96
63|17
63|81
63|32
63|43
63|42
63|35
63|37
63|13
63|76
63|56
63|97
63|54
63|65
63|71
63|23
37|22
37|56
37|99
37|36
37|33
37|78
37|92
37|23
37|52
37|14
37|24
37|55
37|13
37|48
37|25
37|81
37|89
37|83
37|21
37|96
37|15
37|17
37|88
37|51
15|67
15|61
15|29
15|77
15|76
15|68
15|97
15|71
15|85
15|32
15|64
15|63
15|54
15|35
15|87
15|91
15|38
15|43
15|78
15|42
15|44
15|82
15|18
15|72
82|71
82|54
82|36
82|65
82|61
82|63
82|35
82|96
82|32
82|33
82|48
82|76
82|97
82|56
82|29
82|43
82|89
82|23
82|38
82|25
82|42
82|37
82|44
82|13
88|42
88|78
88|71
88|15
88|44
88|54
88|51
88|85
88|97
88|43
88|72
88|91
88|76
88|87
88|18
88|64
88|82
88|63
88|77
88|38
88|67
88|68
88|55
88|61
14|85
14|77
14|68
14|87
14|55
14|67
14|44
14|99
14|22
14|91
14|92
14|61
14|38
14|64
14|71
14|15
14|72
14|52
14|51
14|82
14|63
14|78
14|88
14|18
18|65
18|64
18|71
18|38
18|33
18|63
18|43
18|61
18|67
18|91
18|37
18|13
18|82
18|29
18|48
18|97
18|44
18|42
18|23
18|35
18|85
18|54
18|76
18|32
77|54
77|68
77|97
77|63
77|67
77|85
77|37
77|42
77|38
77|61
77|72
77|18
77|65
77|29
77|44
77|35
77|91
77|82
77|43
77|71
77|32
77|76
77|87
77|64
44|38
44|35
44|25
44|36
44|29
44|33
44|89
44|76
44|56
44|37
44|23
44|48
44|96
44|65
44|97
44|43
44|54
44|71
44|17
44|21
44|81
44|42
44|32
44|13
21|61
21|22
21|14
21|82
21|72
21|92
21|91
21|18
21|15
21|24
21|87
21|67
21|83
21|55
21|85
21|99
21|77
21|63
21|68
21|51
21|88
21|64
21|52
21|78
87|37
87|65
87|64
87|48
87|67
87|91
87|33
87|82
87|29
87|63
87|54
87|85
87|42
87|61
87|76
87|38
87|97
87|44
87|35
87|43
87|32
87|18
87|13
87|71
89|87
89|81
89|92
89|78
89|15
89|88
89|64
89|91
89|17
89|68
89|24
89|72
89|77
89|99
89|55
89|83
89|22
89|18
89|14
89|52
89|21
89|85
89|51
89|67
61|63
61|97
61|33
61|23
61|65
61|17
61|35
61|13
61|56
61|71
61|44
61|96
61|42
61|38
61|37
61|29
61|76
61|48
61|32
61|54
61|36
61|43
61|89
61|25
91|65
91|67
91|44
91|13
91|76
91|32
91|61
91|82
91|63
91|85
91|29
91|43
91|36
91|64
91|48
91|71
91|38
91|37
91|33
91|42
91|97
91|35
91|23
91|54
68|87
68|65
68|63
68|91
68|38
68|37
68|61
68|35
68|54
68|32
68|82
68|97
68|18
68|71
68|13
68|72
68|64
68|85
68|44
68|42
68|29
68|76
68|43
68|67
99|91
99|15
99|38
99|55
99|72
99|64
99|77
99|18
99|97
99|54
99|76
99|61
99|78
99|44
99|71
99|68
99|82
99|43
99|51
99|88
99|85
99|63
99|67
99|87
13|15
13|14
13|99
13|89
13|17
13|21
13|25
13|51
13|56
13|33
13|48
13|36
13|78
13|83
13|77
13|81
13|24
13|52
13|88
13|23
13|22
13|55
13|92
13|96
24|18
24|87
24|77
24|92
24|52
24|14
24|44
24|61
24|22
24|71
24|82
24|72
24|78
24|68
24|64
24|63
24|51
24|67
24|88
24|99
24|15
24|85
24|91
38|43
38|65
38|54
38|42
38|23
38|33
38|76
38|83
38|96
38|25
38|37
38|48
38|35
38|81
38|56
38|97
38|29
38|13
38|36
38|89
38|24
38|17
48|33
48|83
48|23
48|78
48|55
48|81
48|22
48|77
48|89
48|25
48|24
48|21
48|99
48|36
48|17
48|96
48|68
48|56
48|52
48|15
48|88
78|68
78|72
78|77
78|43
78|64
78|38
78|61
78|97
78|71
78|76
78|42
78|32
78|85
78|65
78|91
78|63
78|82
78|18
78|67
78|29
72|32
72|67
72|97
72|64
72|61
72|13
72|42
72|85
72|76
72|71
72|91
72|35
72|18
72|54
72|48
72|63
72|43
72|38
72|44
23|96
23|68
23|92
23|99
23|22
23|14
23|89
23|72
23|77
23|17
23|24
23|55
23|52
23|15
23|78
23|88
23|36
23|21
25|72
25|51
25|52
25|21
25|92
25|78
25|87
25|91
25|24
25|88
25|96
25|83
25|99
25|81
25|68
25|89
25|56
54|83
54|22
54|81
54|42
54|48
54|92
54|24
54|25
54|13
54|35
54|52
54|37
54|32
54|14
54|23
54|17
56|68
56|24
56|99
56|72
56|15
56|55
56|51
56|18
56|21
56|91
56|87
56|92
56|83
56|89
56|96
33|56
33|99
33|96
33|52
33|81
33|88
33|15
33|36
33|68
33|77
33|17
33|51
33|24
33|21
96|72
96|52
96|67
96|51
96|85
96|81
96|89
96|87
96|18
96|17
96|22
96|15
96|99
97|42
97|29
97|14
97|36
97|81
97|32
97|23
97|21
97|35
97|76
97|24
97|25
35|65
35|22
35|99
35|37
35|13
35|96
35|81
35|36
35|14
35|48
35|83
52|92
52|15
52|78
52|63
52|67
52|22
52|72
52|38
52|87
52|77
22|92
22|78
22|99
22|82
22|44
22|67
22|77
22|38
22|55
29|55
29|92
29|56
29|88
29|22
29|83
29|25
29|37
65|14
65|99
65|88
65|13
65|51
65|96
65|33
36|81
36|87
36|25
36|15
36|22
36|18
32|36
32|48
32|89
32|23
32|52
64|82
64|63
64|61
64|35
55|32
55|42
55|82
43|81
43|29
92|67"#;


	let mut hashmap: HashMap<i8, Vec<i8>> = HashMap::new();

	for rule in ruleset.split("\n") {
		let nums: Vec<i8> = rule
		.split("|")
		.map(|s| s.parse::<i8>().expect("Invalid integer"))
		.collect();

		hashmap.entry(nums[0]).or_insert_with(Vec::new).push(nums[1]);
	}
	
	hashmap
}

fn lists() -> Vec<Vec<i8>> {

	vec![
	vec![42,54,21,36,22,33,13,29,35],
	vec![83,67,22,14,78,99,18,92,15,77,52,68,82,55,21,61,85,91,51,64,72,24,88],
	vec![85,67,64,82,61,63,44,71,38,97,43,54,42,32,29,35,65,37,13,48,33,23,36],
	vec![96,81,21,14,52,99,88,55,51,15,77,68,72,18,85],
	vec![63,52,77,85,91,83,22,61,14,64,82,68,51,24,55],
	vec![63,97,76,32,29,35,13,48,33,36,25,56,96,89,17],
	vec![15,99,51,88,21,83,72,56,81,18,92,52,55,17,89,96,87],
	vec![92,22,17,96,78,87,72,14,24,55,81,91,52,83,68,88,18,99,56],
	vec![87,18,61,67,64,76,72,85,97,65,91,68,32,44,29,43,37],
	vec![87,63,72,64,85,38,43,78,76,82,77,67,88,54,55],
	vec![51,77,87,85,67,61,63,44,97],
	vec![35,65,37,13,48,36,25,56,81,24,22,92,88],
	vec![85,29,32,42,35,87,43,13,63,37,71,72,82,76,91],
	vec![56,35,81,96,55,92,83],
	vec![54,68,67,77,61,18,63,76,82,55,97,15,42,91,51,38,85,43,72,87,71],
	vec![99,88,55,15,77,68,72,87,18,91,67,82,61,44,71,38,97,43,76],
	vec![14,17,81,91,55,22,21,87,64],
	vec![92,51,68,96,81,17,88,89,83,22,23,15,14,21,25,99,24,78,56,72,36,77,55],
	vec![68,87,18,85,82,44,71,43,76,54,42,32,29,35,37],
	vec![92,22,88,14,21,55,24,17,25,36,13,23,48],
	vec![92,25,18,68,88,51,17],
	vec![32,17,37,54,89,71,43,33,42,21,25],
	vec![33,36,25,56,96,89,17,81,83,24,14,52,22,92,99,55,51,15,78,77,68],
	vec![61,63,44,71,38,43,76,54,42,32,65,13,48,33,36,56,89],
	vec![14,88,51,87,64,44,71],
	vec![14,92,88,51,15,77,68,87,71],
	vec![42,21,89,83,35,65,81,24,23,32,33,13,29,14,52,25,17],
	vec![17,83,14,22,92,55,51,15,78,77,18,91,85,67,64],
	vec![21,83,22,18,92,67,15,99,64,85,68,87,55,91,78,51,88,77,14,61,52],
	vec![64,15,77,85,92,61,88,83,78,22,18,72,68,52,82,91,87,63,14,99,55],
	vec![89,21,76,35,96,83,17,13,38,25,54],
	vec![77,68,91,85,64,82,44,38,65],
	vec![54,64,91,38,63,51,71,61,85,97,44,18,67,76,43,72,78,15,32],
	vec![51,87,52,85,72,14,83,24,68,15,21,17,92],
	vec![21,83,99,96,36,17,52,81,56,65,25,14,48,13,24,92,55,23,89],
	vec![76,42,29,48,33,36,25,56,96,89,17,81,83],
	vec![77,55,22,78,87,64,61,88,67,68,71,72,51,44,52,82,99,15,14],
	vec![37,33,85,54,44,76,38,23,13,42,36],
	vec![43,25,29,13,33,38,35,97,48,56,65,32,17,54,89,81,44,37,36,23,71],
	vec![81,92,88,78,68,91,85,67,82],
	vec![56,81,21,24,14,88,55,51,77,68,91],
	vec![97,29,65,33,17,83,24],
	vec![38,33,56,43,61,35,76,54,64,32,36],
	vec![63,44,71,38,43,76,54,42,37,13,48,23,36],
	vec![22,17,96,89,14,92,83,37,99,88,29,21,52,48,23,81,35,24,36,13,25],
	vec![77,61,18,87,72,51,38],
	vec![24,92,99,55,51,85,44],
	vec![64,82,77,35,38],
	vec![87,51,15,88,67,52,18,82,72,77,99,71,64,22,44],
	vec![99,88,51,15,78,72,87,18,85,67,64,61,63,71,76],
	vec![23,36,25,56,96,17,81,83,14,52,92,88,55,51,78,68,72],
	vec![52,92,99,15,78,77,68,72,87,91,85,64,82,61,44,71,38],
	vec![13,23,17,14,99,55,78],
	vec![25,17,35,37,44,54,71,81,56,65,32],
	vec![38,64,55,52,87,77,71,99,88,68,18],
	vec![71,38,97,76,54,42,32,29,35,65,13,33,23,36,25,56,96,89,17,81,21],
	vec![82,91,52,83,21,18,14,77,85,61,92,64,88,51,68,99,78,22,24],
	vec![29,35,13,33,25,56,96,89,17,21,83,24,14,52,22,92,88],
	vec![91,61,14,88,18,15,55,99,82,77,64,92,78,87,71,68,51,85,72],
	vec![13,36,35,32,38,76,63,82,37,43,65,42,54,48,64,29,56,97,33],
	vec![61,88,71,68,92,55,77,63,67,51,38],
	vec![18,85,88,63,77,52,38,61,64],
	vec![89,83,99,88,55,15,78,77,68,72,87,18,91,85,67],
	vec![13,17,35,89,48,33,43,54,21,23,38,83,76],
	vec![36,25,56,96,89,17,81,21,83,24,14,52,22,92,99,88,51,15,78,77,68,72,87],
	vec![24,52,22,55,51,15,78,72,87,64,82,61,44],
	vec![55,15,78,87,18,85,63,71,38,97,54],
	vec![37,44,61,32,85,76,71,68,87],
	vec![23,65,89,56,33,21,37,22,48,17,96,25,92],
	vec![24,14,52,92,99,88,55,51,15,72,87,18,91,61,63],
	vec![24,14,22,92,99,88,51,15,78,68,72,87,18,85,64,82,61,63,44],
	vec![64,44,97,43,76,54,42,35,37,13,33,36,56],
	vec![15,87,91,67,82,61,97],
	vec![63,18,32,76,67,82,15,64,91,72,85,68,54,43,42,71,61,87,97,44,38,78,29],
	vec![48,33,25,56,96,89,81,21,14,52,22,92,99,88,55,51,15,78,77],
	vec![89,17,81,83,52,88,15,78,77,72,87],
	vec![51,55,92,77,88,72,68,24,87,89,85,99,52,14,83,81,21,91,15,17,22,18,96],
	vec![76,35,33,13,14,42,81,43,23,29,89,24,21,96,17,37,32,25,65],
	vec![89,25,13,56,78,15,92,48,17,23,88,99,14,51,52],
	vec![21,83,54,33,35,25,13,22,48,37,56,14,36,89,42],
	vec![61,71,29,37,13],
	vec![96,17,24,14,22,92,88,51,77,72,85],
	vec![99,88,55,51,77,68,72,87,18,91,85,82,61,63,71,38,97,43,76],
	vec![65,35,42,32,13,17,25,83,33,23,54,14,24],
	vec![76,54,87,72,61,18,97,91,44,42,65,85,82,71,77,35,29],
	vec![91,87,92,78,21,52,18,24,68,85,77,15,81,83,22,64,72,51,99,82,14,67,88],
	vec![43,32,35,37,13,96,17],
	vec![33,36,96,21,24,14,52,22,92],
	vec![77,83,92,85,81,91,21,68,24,64,82],
	vec![61,71,48,63,44,32,42,65,37,56,96,43,35,76,29,23,38],
	vec![44,71,38,54,32,35,13,33,23,56,96],
	vec![23,17,81,21,83,24,92,99,88,55,51,15,78,77,72],
	vec![42,33,61,67,23,64,25,65,82],
	vec![81,13,48,22,29,23,35,14,21,92,89],
	vec![33,23,36,25,56,96,17,81,21,83,24,14,52,22,92,99,88,55,51,15,78,77,68],
	vec![32,29,35,65,37,48,23,36,56,96,89,17,81,21,83,24,52,22,99],
	vec![65,37,13,25,96,89,17,21,92],
	vec![48,76,13,82,71,97,23,61,56,29,36,33,44,42,25,43,38,54,37,96,35],
	vec![21,99,61,87,68,22,88],
	vec![18,38,54,32,35,48,33],
	vec![97,63,55,78,15,77,68,87,61,18,38,51,71,91,88,99,92,22,44,85,82,67,64],
	vec![56,99,36,96,13],
	vec![56,25,43,76,23,33,64,29,36],
	vec![99,51,68,87,91,82,63,44,71,43,76],
	vec![82,54,85,91,29,64,33,61,42,37,97],
	vec![56,42,17,33,97,43,63],
	vec![77,17,21,99,52,85,15,91,78,88,14,67,51,24,92,22,64],
	vec![55,51,15,78,77,68,72,87,18,91,85,67,64,82,61,63,44,38,97,43,76,54,42],
	vec![87,78,67,18,42,61,55,82,91,71,85,97,15,72,43,76,63,44,64,38,54,77,51],
	vec![37,44,64,13,29,97,25,82,61,67,36,35,43,63,23],
	vec![33,23,25,56,96,89,17,21,83,14,52,22,92,99,88,51,78,77,68],
	vec![33,36,32,35,48,23,24,21,22,17,99,83,25,37,96,29,81,92,14,65,52],
	vec![76,32,71,42,33,82,97,36,54,44,35,13,29,43,37,63,48,96,65],
	vec![65,37,13,48,33,23,36,25,56,96,89,17,81,21,83,24,52,22,92,99,88,55,51],
	vec![42,32,29,35,37,13,48,23,36,56,96,21,83,14,52,22,92],
	vec![42,43,25,65,23,37,61,44,48,29,76,33,36,38,54,97,96,89,35],
	vec![52,99,55,51,78,18,64,82,63,71,38],
	vec![36,96,89,17,81,83,14,52,88,51,15,78,77],
	vec![96,81,51,15,99,37,24,36,13,17,33],
	vec![21,83,24,14,52,99,88,55,51,15,68,72,91,85,67,64,61],
	vec![25,51,81,22,99,56,18,52,78,14,87,92,15,77,68,83,55],
	vec![81,77,92,21,51,15,99,96,56],
	vec![71,43,65,82,54,42,13,36,25,61,23,33,35,63,67,38,76,97,37,32,29],
	vec![37,76,63,61,96,56,25,35,13,36,43,29,82,32,48,33,97],
	vec![35,52,23,14,96,42,89,21,81,83,76],
	vec![48,23,36,25,56,96,89,17,81,21,83,24,14,52,22,92,99,88,55,51,15,78,77],
	vec![44,61,85,92,91,77,15,64,51],
	vec![37,13,48,33,23,25,56,81,83,52,92,51,15],
	vec![56,78,96,88,92,36,23,17,25,52,48,99,24,89,55,21,15,81,51],
	vec![64,22,67,55,97,72,15,18,68,38,88],
	vec![85,64,68,44,54,87,76,91,82,63,88],
	vec![85,38,78,82,18,87,91,32,77,43,76,72,15,42,51],
	vec![22,92,88,72,87,91,67,82,61,44,97],
	vec![55,18,54,38,87,88,85,15,76],
	vec![83,24,14,52,22,92,88,55,51,15,78,77,68,72,18,91,85,67,64,61,63],
	vec![42,32,35,65,37,13,48,33,23,36,25,56,96,89,21,83,24,14,52,22,92],
	vec![32,17,29,83,48,14,21,25,52,42,96,33,37,65,76],
	vec![35,89,81,14,32,83,29,65,52,25,92,13,22,36,33,48,37,42,24],
	vec![81,42,35,36,89,54,71,25,32,43,76,37,38,44,65,23,17,33,56],
	vec![21,18,87,52,22,56,51,15,77,92,25,55,72,99,78,96,14,83,24],
	vec![56,25,37,81,38,89,21,43,65,83,33,36,35,96,48,42,29],
	vec![55,15,77,68,87,18,91,85,61,44,38,97,43,76,42],
	vec![22,99,72,83,67,52,61,21,68,14,87,18,77,51,64],
	vec![92,55,85,14,22,17,52,89,78],
	vec![88,68,18,14,81,22,91,52,83,99,72],
	vec![36,17,29,83,96,65,54,89,43,35,56,23,25,13,81,76,14],
	vec![88,51,78,63,43,76,54],
	vec![91,67,82,61,38,97,43,76,54,42,32,29,35,13,48,33,23],
	vec![67,63,35,71,54,68,37],
	vec![85,35,33,32,61,76,65,54,37,67,91,42,97,48,71,82,38,29,13,44,43],
	vec![77,15,81,89,96,25,14,92,78,22,51,24,21,83,56,18,88,99,68],
	vec![23,36,25,22,92,15,72],
	vec![23,25,54,36,65,17,14,76,56,96,35,48,81,24,37,21,89],
	vec![85,64,61,38,32,29,65],
	vec![56,89,17,24,14,92,99,55,78,18,91],
	vec![18,85,61,63,44,71,38,48,33],
	vec![81,21,14,52,92,88,72,67,82],
	vec![55,51,15,78,77,64,63,44,38,43,76,54,42],
	vec![76,54,48,23,96,81,83,24,52],
	vec![21,83,13,14,22,96,25,37,56,81,36,92,42,89,23,32,24,33,52,35,17],
	vec![42,13,91,38,43,82,35,18,87,29,37,85,64,67,54,32,63,76,72,61,44],
	vec![23,22,29,21,24,36,14,99,89,56,88],
	vec![77,56,15,33,48,24,81,22,92],
	vec![78,87,85,67,44,71,97,29,35],
	vec![61,15,78,63,77,71,44,68,91,87,32,85,72,43,76,82,29,97,42,54,18,38,67],
	vec![99,18,81,82,87,24,78,51,83],
	vec![54,65,48,36,14,52,22],
	vec![63,44,38,97,54,42,32,65,37,56,96,89,17],
	vec![83,88,68,72,81,52,55,36,25,22,17,92,87],
	vec![44,38,97,54,42,37,13,33,23,36,81],
	vec![61,18,67,64,38,35,65,85,68,29,37,63,71,42,91,82,97],
	vec![65,54,23,35,89,43,29,38,36,76,81,21,32,17,56,71,42],
	vec![96,78,55,91,17,77,85],
	vec![64,82,61,44,38,97,43,42,32,35,65,37,48,33,23],
	vec![23,36,25,56,89,17,81,21,83,14,52,22,92,99,88,55,51,15,78,68,72],
	vec![97,43,54,42,32,29,35,37,13,48,33,23,56,96,89,17,21,83,24],
	vec![54,65,36,23,71],
	vec![13,44,33,63,48,97,25,38,64,82,61,56,76,42,54,65,71,43,23,35,36,37,32],
	vec![65,13,48,23,36,56,89,21,52,22,88,55,51],
	vec![68,51,14,83,64,18,77,17,91],
	vec![76,37,38,25,13,33,71,21,42],
	vec![21,92,81,87,24,56,51],
	vec![48,83,35,37,81,97,89,54,33,29,36,13,96,23,38,21,65,17,76,56,32,43,25],
	vec![13,48,17,83,14,15,78],
	vec![18,22,92,82,68,87,24,52,44,99,67],
	vec![77,99,87,92,22,51,52,68,71,14,82],
	vec![22,67,83,63,14,15,82],
	vec![64,44,97,61,67,32,37,65,38,29,35,48,71,36,54,76,85],
	vec![24,14,92,99,15,78,87],
	vec![63,71,76,54,32,29,35,65,37,13,48,33,23,36,25,56,96,89,17],
	vec![42,96,71,35,25,65,23,33,44,81,32,97,29,76,13,54,43,56,17],
	vec![87,91,67,82,44,71,38,97,43,76,29,35,65,37,48],
	vec![33,38,13,48,65,43,61,82,37,76,54,32,35,44,67,29,85],
	vec![24,52,22,92,55,51,78,68,72,18,91,85,64,82,44],
	vec![63,48,85,67,38],
	vec![42,32,35,65,13,48,33,36,25,89,17,81,14],
	vec![63,44,71,43,35,65,37,13,23,36,17],
	vec![25,21,97,33,54,32,76,42,37,17,35,65,89,81,83,96,36,23,48,43,13,38,29],
	vec![42,32,29,35,65,37,33,25,56,89,17,21,24,14,52,22,92],
	vec![37,48,81,21,83,14,22,88,55,51,15],
	vec![55,51,77,68,87,18,91,85,67,82,63,44,38,97,54],
	vec![35,65,13,33,36],
	vec![61,48,44,64,43,36,82,42,56,29,23,54,33],
	vec![92,99,88,55,15,78,77,68,72,18,91,67,64,82,61,63,44,71,38,97,43],
	vec![32,29,33,23,96,83,22,92,99],
	vec![63,55,61,92,87,64,88,85,72],
	vec![67,92,77,64,52,51,81,91,17,85,21,22,14,78,15,88,18,72,83,55,68],
	vec![36,78,92,22,51,87,99],
	vec![37,13,36,56,81,92,88,55,15],
	vec![29,35,37,13,48,33,23,25,56,96,89,17,81,21,83,24,52,22,92,99,88]
	]
}

pub fn five_a() {
	let rulesmap: HashMap<i8, Vec<i8>> = rules();
	let mut lists = lists();

	let mut value = 0;

	'outer : for list in &mut lists {
		for (i, num) in list.iter().enumerate() {
			let rules = rulesmap.get(&num);
	
			if let Some(rules) = rules {
				for rev_i in (0..=i).rev() {
					let check_num = list[rev_i];
					if rules.contains(&check_num) {
						continue 'outer; 
					}
				}
			}
		}

		value += list[list.len() / 2] as i32;
	}
	println!("Sum of already ordered centers {:?}", value);
}

pub fn five_b() {
	let rulesmap: HashMap<i8, Vec<i8>> = rules();
	let mut lists = lists();

	let mut value = 0;

	for list in &mut lists {

		let mut reordered : bool = false;

		let mut i = 0;
		while i < list.len() {
			let num = list[i];
			let rules = rulesmap.get(&num);
			let mut lowest_broken_rule: usize = i;
	
			if let Some(rules) = rules {
				for rev_i in (0..=i).rev() {
					let check_num = list[rev_i];
					if rules.contains(&check_num) {
						lowest_broken_rule = rev_i;
					}
				}
			}
	
			if lowest_broken_rule < i {
				let moved_element = list.remove(i);
				list.insert(lowest_broken_rule, moved_element);
				reordered = true;
			} else {
				i += 1;
			}
		}

		if reordered {
			value += list[list.len() / 2] as i32;
		}
	}
	println!("Sum of reorder centers {:?}", value);
}

//[42, 54, 21, 36, 22, 33, 13, 29, 35]
//[54, 42, 29, 35, 13, 33, 36, 21, 22]