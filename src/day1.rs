#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let mut left: [u32; 1000] = [0; 1000];
    let mut right: [u32; 1000] = [0; 1000];
    let bytes = input.as_bytes();

    unsafe {
        for j in 0..1000 {
            // set start of row
            let i: usize = j * 14;

            // Parse first number
            let lhs: u32 = (*bytes.get_unchecked(i) - 48) as u32 * 10u32.pow(4)
                + (*bytes.get_unchecked(i + 1) - 48) as u32 * 10u32.pow(3)
                + (*bytes.get_unchecked(i + 2) - 48) as u32 * 10u32.pow(2)
                + (*bytes.get_unchecked(i + 3) - 48) as u32 * 10u32.pow(1)
                + (*bytes.get_unchecked(i + 4) - 48) as u32;

            // Parse second number
            let rhs: u32 = (*bytes.get_unchecked(i + 8) - 48) as u32 * 10u32.pow(4)
                + (*bytes.get_unchecked(i + 9) - 48) as u32 * 10u32.pow(3)
                + (*bytes.get_unchecked(i + 10) - 48) as u32 * 10u32.pow(2)
                + (*bytes.get_unchecked(i + 11) - 48) as u32 * 10u32.pow(1)
                + (*bytes.get_unchecked(i + 12) - 48) as u32;

            // Add to vectors
            *left.get_unchecked_mut(j) = lhs;
            *right.get_unchecked_mut(j) = rhs;
        }

        left.sort_unstable();
        right.sort_unstable();
    }

    left.iter()
        .zip(right)
        .fold(0, |acc, (lhs, rhs)| acc + lhs.abs_diff(rhs))
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let mut left: [usize; 1000] = [0; 1000];
    let mut right: [usize; 1000] = [0; 1000];
    let bytes = input.as_bytes();

    unsafe {
        for j in 0..1000 {
            // set start of row
            let i: usize = j * 14;

            // Parse first number
            let lhs: usize = (*bytes.get_unchecked(i) - 48) as usize * 10usize.pow(4)
                + (*bytes.get_unchecked(i + 1) - 48) as usize * 10usize.pow(3)
                + (*bytes.get_unchecked(i + 2) - 48) as usize * 10usize.pow(2)
                + (*bytes.get_unchecked(i + 3) - 48) as usize * 10usize.pow(1)
                + (*bytes.get_unchecked(i + 4) - 48) as usize;

            // Parse second number
            let rhs: usize = (*bytes.get_unchecked(i + 8) - 48) as usize * 10usize.pow(4)
                + (*bytes.get_unchecked(i + 9) - 48) as usize * 10usize.pow(3)
                + (*bytes.get_unchecked(i + 10) - 48) as usize * 10usize.pow(2)
                + (*bytes.get_unchecked(i + 11) - 48) as usize * 10usize.pow(1)
                + (*bytes.get_unchecked(i + 12) - 48) as usize;

            // Add to vectors
            *left.get_unchecked_mut(j) = lhs;
            *right.get_unchecked_mut(j) = rhs;
        }
    }

    let mut freq: [usize; 89999] = [0; 89999];
    right
        .into_iter()
        .for_each(|value: usize| freq[value - 10000] += 1);

    left.iter().fold(0, |acc: usize, value: &usize| {
        acc + *value * freq[*value - 10000]
    })
}

mod tests {
    #[allow(unused)]
    use crate::day1::{part1, part2};

    #[test]
    fn test_day1() {
        let test: &str = "38665   13337
84587   21418
93374   50722
68298   57474
54771   18244
49242   83955
66490   44116
65908   51323
71704   87343
38086   70349
14187   91077
20532   16203
65614   30387
11304   65867
25362   70932
72055   26767
49666   31944
66173   31095
40837   26167
35579   85581
61232   50722
52091   67378
18314   15102
35195   61300
54331   72814
33400   72890
79533   70349
77040   19254
26223   16941
88815   52540
45794   61915
13890   27233
19261   96273
90088   40968
42316   43945
25415   83221
28956   50722
52792   27899
88503   29185
68711   36941
77367   63468
50643   31908
55410   58164
33112   20034
23414   80569
27170   27626
49843   32511
73574   69852
71297   54016
41032   17525
54311   31743
67629   67059
18536   98194
10030   63468
20745   46576
57680   67071
90422   16941
20305   70932
15917   19741
99356   86711
51850   54713
76292   92167
98063   13065
12191   25063
97360   72545
33211   57474
42295   31908
79769   50722
51924   52337
43683   76152
39179   71113
60185   16467
57990   77323
77295   46576
24790   91936
28283   13191
99413   85581
56701   27233
41134   93737
26842   44188
90263   70349
96726   83955
95896   19235
30992   43359
12686   78261
10520   38756
92588   11945
89222   88586
48462   70932
77694   46576
29215   51794
42318   57474
84819   63468
91225   67702
31832   19567
38087   43275
81945   67194
77457   63696
61439   63696
40369   74690
88405   67702
66223   54787
95105   45844
90075   12214
69993   34197
48011   34540
29342   50722
85052   11794
98660   71704
94197   97300
93139   68070
68878   76000
71619   27922
46977   48818
11904   63858
47001   16770
82065   34197
29721   82754
31105   63940
97028   78838
29016   34194
69812   97395
36533   57474
25894   44188
26266   23023
90634   58164
46563   91672
79665   95153
80709   15843
26878   27870
56666   34536
34285   34197
28183   86861
96099   97197
88880   97395
23770   50339
81640   33646
71390   63858
54825   88538
36741   57877
80851   51129
21007   70932
81349   13098
57834   57695
27735   15100
84023   83399
38920   53026
17430   65867
99043   79360
94223   84028
94457   66299
72780   93015
32444   68923
62372   31856
51087   99720
93830   98194
42686   83955
57610   34197
45047   65867
98633   89719
63847   77323
68513   57474
49453   65867
55946   28322
50856   97261
45789   16941
72205   17456
31505   66299
64173   66299
98155   50955
70753   76627
29611   70932
11137   42292
32909   34197
63679   11794
83794   44116
99292   70016
54713   68852
11146   44116
37838   59069
13697   48336
25542   46576
83440   65867
78504   45977
22763   81348
27994   16941
30091   16467
62383   85581
18984   11794
16901   66299
92207   33901
73811   70932
81337   17817
67498   63696
47293   78566
50722   20178
18778   71704
97503   65321
79922   50722
53969   16941
15725   90452
27233   17456
15530   98194
90382   97161
99242   65867
23566   79922
21127   21048
37703   81444
52108   57474
36769   70932
33909   82199
48827   76724
76362   71704
59133   14778
39132   80569
99081   57474
76480   66540
25596   34197
50235   11525
28581   76932
16076   31849
17720   31475
71042   20206
15240   76152
19831   98187
44116   67702
63696   45629
41286   97395
98194   20678
50004   64228
41887   34197
76711   70349
99317   80569
13613   65867
16137   63696
42410   43945
34169   45927
44621   21038
72734   63696
61396   84310
36759   66120
84702   65867
85816   18236
24559   54693
22316   34536
52032   18454
15164   39543
85163   16941
47197   87792
14939   55106
17851   63858
39134   75532
93005   34536
12738   88547
70180   23031
36942   29745
64429   28730
13719   62682
36494   76152
70321   58692
24536   67702
43234   50722
88874   54713
96469   43346
98916   38515
70083   62273
58578   11794
80256   16203
37111   77602
68334   82082
46279   46576
30153   34746
37211   98194
50865   79922
98773   82616
59655   56203
34197   76152
65623   93234
39161   79718
92822   50722
97609   16798
28771   85581
72903   28436
81696   56694
66299   97864
16467   69051
66035   16941
33128   81165
92747   27870
40431   15562
40139   79922
28567   56965
69276   61616
50958   39665
95107   85581
98949   96754
97628   83955
75515   15021
38185   80831
64872   34197
69156   97395
17320   56612
29089   34197
19121   50812
69225   60185
70618   83315
23028   28786
25084   98236
74004   20010
34216   17456
71767   44188
11957   79477
43970   71470
50265   45492
65867   84267
82785   59825
28455   65867
13521   77323
52870   24028
81192   76152
52922   50955
15786   27233
34277   57816
50204   77323
68620   98054
56877   83955
64217   57474
67705   85581
44264   91864
15673   76152
97395   27233
89037   46576
74355   54515
60861   65867
35556   16467
48771   82369
83628   44116
76214   99720
58893   73397
99933   65077
86446   63468
75068   66574
85752   61667
70300   97395
61613   63858
98122   63696
41327   45689
65845   73173
70915   82199
75864   70932
38907   83673
52791   16941
99407   50281
83332   62140
30046   41764
56280   63858
32820   59359
70360   76152
67702   50722
97945   49745
57069   10905
58962   77365
68107   15986
84616   97395
39475   16203
14462   57474
10512   88977
74047   17456
32711   12367
67494   66299
90232   40939
10100   28855
78048   56287
13711   49326
21916   81443
62495   46576
37116   63468
80569   81942
74392   83955
26299   63071
58164   63696
93301   57474
31492   77650
39217   44247
34327   70932
18995   39486
24310   94657
66983   99720
28500   17603
34429   77987
64239   41386
24999   99720
74548   16203
25193   40228
87634   16467
36216   74814
10539   89684
22373   76152
28271   95381
88135   34536
99042   13284
33745   70042
12552   10822
42209   91001
96584   88339
66574   87712
78799   85581
57785   99568
63996   27233
32612   85581
74135   63468
81597   76152
42352   16203
54028   50822
69223   34197
21961   96505
71713   34536
42402   59457
10187   99720
41391   65867
14008   63696
27601   88355
86754   83955
47563   50722
88397   61525
95933   97395
51110   76908
29218   98194
81675   14234
80008   76613
26022   88164
39436   46576
48267   34536
51817   99848
11794   85581
11942   91990
69074   84725
18276   34197
61242   85690
28019   36351
91583   85990
68925   58164
32977   16203
94066   80569
53400   91931
76152   72545
71159   78789
24577   73397
36582   86986
18545   46576
70021   45875
48132   46576
32662   63468
74173   34197
59697   35602
69712   75255
55124   68594
86734   15493
48215   29682
64457   46576
85581   47129
92549   32388
86765   77920
44235   56256
50083   19461
50955   49307
15649   97794
94781   63696
21206   57474
34265   45507
40811   11794
87874   99720
70458   90059
92312   46801
36568   79557
75301   38349
60353   41935
12017   63468
97966   29539
40099   57474
45330   64697
11169   50722
52723   16941
46147   85581
53464   97395
12372   16941
68430   17456
76667   17456
18478   19709
33036   33716
37748   72663
57523   21097
87808   27704
19190   83955
90952   57652
46734   65867
78907   73446
96825   11806
78394   17236
60807   63887
64419   63696
40450   98194
16354   63468
69859   69643
13174   77323
85959   99060
88893   76152
92814   65729
16069   98551
58002   27233
20706   71704
17456   99720
83405   44116
80504   27233
59608   94865
33397   60450
36077   17910
25987   65867
65373   77323
37740   43945
38356   66574
85528   49178
43918   11461
82475   99720
65544   52765
31908   40160
85860   48252
35879   39980
68599   93267
14439   24271
73065   96330
67664   11794
82582   31908
47118   70349
93925   71431
39481   67702
98663   54438
37035   40058
50493   99720
78937   46576
48997   16941
26389   23726
76502   91617
54048   84837
61512   83104
72545   91243
13520   88435
16203   27033
47959   97395
94130   11794
88634   85581
53658   26617
97819   63858
36711   61581
87772   33784
61876   66299
51529   70308
35807   88824
44953   80821
70153   79922
65079   34536
28302   71704
59791   39097
50031   49001
49911   54999
61691   16941
79056   94143
82950   88152
59675   63696
21426   27902
58151   31908
26165   29349
66334   69031
64251   77323
63877   66299
14334   22943
90265   79903
12748   70354
98964   17017
11779   76152
40927   49764
29347   83807
18654   23753
41982   79922
41061   51130
99743   76574
74706   94894
90649   20183
28087   87075
61623   63858
35482   30426
86232   71704
49404   34407
48773   33917
22883   40378
83697   78156
34536   17456
44723   99720
46576   69206
29335   46576
79309   34536
10842   57474
37354   27233
59095   63696
88910   88060
31145   30503
75800   99469
93227   20721
30615   40084
92536   16246
42177   57160
71915   64478
54908   96661
52410   44116
58860   55187
47343   23616
83166   11794
28695   88377
30502   76748
29426   11794
86870   63468
17529   89533
65007   71704
78234   52721
23560   34536
87097   10822
22813   57474
62736   64591
29232   11670
81273   16872
63464   27233
40338   63468
62952   34536
95434   97809
63468   83955
56391   57474
98254   88672
97351   98194
63858   66299
80354   68260
98241   25344
67372   85581
51069   27233
14868   22589
10031   66299
97140   15068
99808   24686
70161   70932
81198   66299
60987   82199
95339   16467
60207   64182
65344   99769
42950   57474
51281   67702
79929   70492
51354   76152
14166   73397
82199   76152
39919   66473
57217   58383
34854   71106
63864   77393
43075   27233
45248   58919
46089   70932
20772   63858
70932   73397
86122   67702
15946   24462
44156   88833
63406   90699
44895   44116
90340   63468
91379   36236
51114   79922
70653   54713
82940   31152
71528   42570
87955   66299
28669   88159
45992   63468
97984   34536
85927   34536
59864   55650
81438   17456
70025   70932
38903   10947
58887   44116
39857   54321
13712   96493
44188   70627
40114   24057
44974   43945
25231   46576
88262   63858
35812   80569
95724   43945
52221   99720
71914   60089
66450   16093
91063   57985
18039   76152
49797   11794
60130   20342
83369   77446
94570   97395
68981   18718
24212   30602
15349   33310
99577   24360
10755   46576
85659   16941
49805   66632
60370   16941
41005   11660
48898   92579
63523   85581
86663   11794
21080   38278
19056   84772
56722   22930
55163   17456
19443   86124
37944   66574
49518   40633
97379   11165
88699   46576
68077   66299
79404   76152
45236   50722
59604   58164
34844   63696
16235   43945
86716   55329
82314   34197
40678   38019
20081   57474
49118   86999
99420   22991
71525   27223
45626   76152
42374   44116
63866   50824
29757   85581
56979   99720
73653   71704
43214   41698
37922   60188
48491   96843
12177   57474
83955   63858
57390   58164
42917   79922
64244   63468
89659   11450
65030   63858
15320   16410
59298   35208
64962   77954
79362   80743
58454   20071
53455   57474
87564   81411
10856   20535
80046   55549
26163   36603
85881   96592
31247   66299
96730   40912
96991   62489
50081   37795
53175   16203
58546   58164
88985   51240
38660   50722
57474   85581
10014   63858
85636   11794
39376   33379
30210   82145
73530   17838
45106   97255
33710   78685
86137   55699
51447   43945
93857   35385
10519   44188
67406   37808
12115   16203
81188   74964
22840   75860
77432   13166
79648   24017
27118   44116
33846   11794
57622   10822
77323   50436
97115   54713
36521   89820
17575   28629
31306   46576
28458   41012
97806   64446
75061   65867
24370   63468
85540   67702
68803   54097
79880   63468
49584   85070
28231   43581
69126   45784
68684   75365
97489   83924
21553   45897
18300   11794
93378   61443
18477   66176
65798   65125
81412   16467
23459   65867
52036   63858
97430   67702
60092   46098
38308   84883
58067   71647
62704   70932
50105   54713
59790   79806
79629   70349
27870   67066
81277   82199
60415   70349
63609   66190
65271   67702
90053   49879
92015   13892
61636   16941
39029   97395
88319   11794
90430   16941
50684   73350
49781   57474
90672   99720
54006   23608
18335   97395
68550   83955
17242   98714
56443   78472
13025   78528
88580   57328
57505   20525
73928   63858
50645   79922
34654   79349
16150   79038
59245   99720
41292   17670
37153   16827
40119   70349
15038   16653
51995   51842
72008   63696
39351   20612
94307   77323
65374   63468
33818   80569
35632   36792
13340   66299
64037   11794
11327   57198
94027   70932
74680   72545
90499   69512
84533   54713
70349   58164
86058   83955
70971   33938
37024   34536
28948   52419
44022   14761
75811   72545
81326   64922
61416   73312
34737   24227
55533   61805
28249   65867
16982   80433
86902   12617
97265   71704
13441   78189
53639   12907
59068   11794
93717   94858
92354   92976
71611   76984
18339   88783
80948   46576
41117   19375
82662   38645
46322   16203
63147   77753
35846   43313
13358   76152
29798   27233
35165   76152
64871   18525
59660   14589
77415   26565
76862   16941
28351   59870
83371   83955
19365   55304
61679   99720
25350   99250
28686   26917
11232   54597
25291   73397
18993   34536
35849   35298
59974   81973
93998   34536
23836   83955
21732   11794
17870   67702
74048   82199
90162   16203
66432   68461
46585   46576
80339   35328
19048   52750
91316   20240
31309   30857
62174   35350
11790   46576
51587   16203
38597   39285
44150   65867
33021   34536
39620   47484
86129   85581
53864   63696
90229   99720
48614   58164
81933   97395
62593   46107
51827   65867
54166   50722
87787   30910
76553   44116
99790   69614
41931   43945
29481   70932
85717   84033
92701   33388
99720   28016
73397   67702
41967   26222
98617   44582
84742   96386
74648   43461
10822   99720
27527   61207
57643   13131
54079   95773
16272   70349
55222   70932
38368   63858
16765   27233
20650   13111
99345   34536
81256   58260
85730   72175
83538   70258
55581   46488
45860   54713
20126   77323
87771   14424
43945   66356
38415   70932
76611   27233
60655   34111
38694   99720
74603   54504
70755   30309
41002   27233
92389   89555
54955   65867
57931   37576
71840   74455
14770   40677
58532   11794
38395   17456
16941   32946
10544   70349
64876   16941
56992   50668
50084   16941
79209   60704
39667   36867";

        assert_eq!(part1(test), 1830467);
        assert_eq!(part2(test), 26674158)
    }
}
