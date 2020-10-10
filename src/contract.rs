use cosmwasm_std::{Api, Binary, CanonicalAddr, Env, Extern, HandleResponse, HumanAddr, InitResponse, Querier, StdError, StdResult, Storage, QueryResponse, log};
use crate::rand::Prng;
use crate::msg::{HandleMsg, InitMsg, QueryMsg};
use crate::state::{config, config_read, State};

use sha2::{Digest, Sha256};

const WHITELISTED_ADDRESSES: &'static [&'static str] = &[
    "secret1t3mfzyvuyj0ngwz0fwyerauc7hhv3z92d8xexd",
    "secret1srkur5fu3d8eyvuyflcehqytyjfxsrts9ne7yp",
    "secret13rxksn23ehe6ulu3h2u95t6jtnz64k8fw20pef",
    "secret1y9tqm9vyyfn3ju68rj0d5ud757d9mtvlnhjalk",
    "secret1ud5d7zfmejk7n2w6h9xuv38urk0nzdjv7hq98u",
    "secret1xq45dgwmvzq6yea3phq6z96attlp6epzm8d6cu",
    "secret104t5rwlm3nzw83gl7g7ywtzass8mj6f8w9h844",
    "secret14c4gvhg6qqgy8xj8n24emfx42f2zyf5nlsuwvm",
    "secret1hhnzxh480h0e635x0f4sz8d509vx7ckp82x7ys",
    "secret1ue9mwe5fjfnshvme570u6mu30gffqq28q8spvz",
    "secret1c92e4ksaygurz9nmce6ghsw9vkwqu5umppjqlq",
    "secret1ysfycalvgne6enqp6jy3lhpsvc5cayul8gjmwz",
    "secret1g4n2a7q5mehdz0yur3h8tg9hnw0ysd7xcj6e4p",
    "secret1vzpdd80zzawm75t6e24z9mt04kqlync79yhxkh",
    "secret13c7zd07nxpnhhl7edpl5stnmzp2lgcw4dy5jyg",
    "secret15t6ngtmc0eyln37r3qm7p2h809el7j87pqa5ls",
    "secret1yvr42cu2nqwsv7dxerdjek276w8wgkl6c9e3xm",
    "secret1n8d45d0f2fvncsmqw3jhc805vtkm654kkvw68l",
    "secret13nmrlzy08qnxly57xyl359a0ly242nkut28mmq",
    "secret1396vzktm4nje2kzkhwkkdwvzaxr9tmrmgqn746",
    "secret1n2jf0jpq6mf2g2fs6ekccd8udw3ary89mvltxk",
    "secret1lkwyls7agk2wqn0mlyjm0zygunjcgqn39wepeg",
    "secret1sxn5tt4nx5jjcwpldk2l2svz6zqxj7268dzzcu",
    "secret1ha5wr0wwkkcqqysnahwtdysan7vg2cxuw4fz7g",
    "secret1gxyd4x28j6rjxdh04mvkf7zntfw5ultgvrn4kt",
    "secret1p0qdzu37wvrjra3m3f5as6vz0rs6e2kr4n6065",
    "secret1xxs9lpnp2g29ll046jurkclcnfx7dakl5h0k37",
    "secret1xgdz0urxr73wyl83r96qtj33lwas4345le2k7e",
    "secret1n3u52tnfye4fajjsdzgfhytf4a0tq4c8hkxj50",
    "secret18397u7deksz95fcus5z80ydfvzemsaacz9eksh",
    "secret1hdq38cjysvluumedd3fpdlyv98n2fhxhqy9dv4",
    "secret1ejzfen4k57vkhpqk6pywvfz4e2r3atcssax0ku",
    "secret1qm9p5udht97n22vzgm5apf48j6j78358qrkjpa",
    "secret1y99mhk98gjhdj9q9y7a4ujp599n4hrvehsrzaw",
    "secret1kzlsxpuuqaah0dq63tpkx3zdq7nnt4nygympf7",
    "secret1muegpuxz6wqsnhqt02mj5h2nhfxk2p0vfcpz0j",
    "secret1367ty4dl6lyfwswk47a2qagzulesg7s9s2ze2z",
    "secret19jjsse2kulg9yna4vgqhz7dhme8lucenfylz9f",
    "secret1965uupkn5jyeyx8y7n5ha5dt69tazfu9wt7u39",
    "secret1vqgqhmr2shxzmwu9l6xnwdjz5zevz9afqdhcmx",
    "secret1txtfjr5h44lwm8c533cd4mf9s906623jjjd9qg",
    "secret18tgpcerhn50m2pqy9nmwshst8xva9eaert84up",
    "secret1zk44ue8vk3pyqx89xnqvlnvsjysl3dd98knlt4",
    "secret19nvkker7v3g6k924sfkzam6mp2e8upst0rfwkw",
    "secret1gceeftnxzzet75s6y8m44ur4ts02tplqu5gvvk",
    "secret12lh70wrj9gj5wvuxfq5e6crtmk3jzsmx8ced7z",
    "secret1r62qllyxls8j4cd66rkl2pfrw9fn03whm7pyl6",
    "secret1nkqutq8v3espqe7gycatlens70p8d9702v5kr3",
    "secret1sd4yz2v7juypvf2mmwhza9a9ars49p4zg0puu4",
    "secret19gzwxjmcqx0anqa7vsdx4m26wsulkewrrcm7nm",
    "secret1qfefjq02pshjuqt9zlymul72226s5qc684s7u4",
    "secret1a266ukugw7x4yvy576fyx6g7k68nvtkrlm7592",
    "secret1tvg4wakwydsehfk4arapyvakq7daclkekcx2xu",
    "secret13l88ven2zkch796n7qr35rrt80k6y5ljckhfsx",
    "secret18ql39enr36nvsk7ewwlujpchmzlrn8k7qxqkzx",
    "secret1gx3zg69eursppd07sdz7u8h63z4rzz3plhp26p",
    "secret1nfm0vzc0k2m20ktcp4p9px7sqhwn7zy8r5qnx3",
    "secret10y2ulpkmrz5nr75k8ktcqjquz646vljgf3s73s",
    "secret1vvy7q8rkqems5qnmsly5xzaqnr82zel07ma8ud",
    "secret12z9k00k7dje9vnpfkkqsr0e7fy795dqjvvg5cu",
    "secret18ty8a3tn0p2rcfnj2jn937w7d35ekpgfqzc5w6",
    "secret1z62kvftq5d9xpfn0vn4vwaayarkzp7dtm6rnke",
    "secret128r0r67ls3nmtywyyuykfsmcck2m75qh4n3838",
    "secret1nlga86tqwutweug5amy3cpxjss0ydwv9etare5",
    "secret1yrlkfnh509ynz8k698jctq7jft9u5faun4etf8",
    "secret1735phwvhrpuhc024pw9x7qktp3ydpy0f7k2lq9",
    "secret1frh9yxruwzds4t66fx8czcrxh4q743l8v4xme3",
    "secret1d9d7p57am3a7lapdwp6qsqrds66cqzk26scxl0",
    "secret143ruzmscls4vveu0y4ujljv5089ahx8uh7t5qp",
    "secret14fs760a6v7p3098sqmje4ehl6rdazf7egzzf9n",
    "secret14n53kz580s5072zm24nu2k780ejp6vfrqcx07z",
    "secret1u724qv3m8vqn5cp5agkpqfnqezjq9xfdqdcm2k",
    "secret1h9tmc6p8nyxa3sd4esrjg9uwgdzsc306v8sa8v",
    "secret1aezwgw8vawv22hnh7wms0pkaggjnmfchuqrqzf",
    "secret1gupwn6t5xvsu9uej6qzs8cuyqss5z25huyc5ys",
    "secret1utzd6r4095925z6c6luelu02l572cdgt54qeph",
    "secret1jw29qydk5d5usf2tnpnmasgdd86ufltzqsupda",
    "secret15q8hk2majr3rsxvswejn7hc3cd5f9x04yykdcn",
    "secret1ggf8f85wx6jqftr0shldlvqqpvglmw648rqs4e",
    "secret15hcj7us0n5e37c8zfl6l4fq6y5r8kslwe57wqw",
    "secret1p0ku8k2203ya55s7hzsv4t8v8yhxf2wyexj6de",
    "secret17099kr2d22x5e2etujv9htmxl0ykhh7jz0xvxx",
    "secret1vv3gxjqvq9lmk8du4rl4z5gtlecrfwyczrz7eu",
    "secret1elggazmwfzrc6dgj9knr87n0y7pfqnr5v03typ",
    "secret1n8z6a070tqn6gjc4r7v72g6ywpy44nlu0055jj",
    "secret1jy2f0eu8u4x5ax4p9lacvsh82xj9yuhwlharwp",
    "secret1c2qt45ld8k7mxagj7p4rt3ss6nux3q2wk7l6gm",
    "secret18p4fjj20672656h83lgqzc494ufxult3yekvt0",
    "secret1420hn0qxmgsj7qma5mncwxgmcsz03570dj7k9p",
    "secret1jg45530ykqnr4x234eh6k3u6pc2l84wjfxxz8k",
    "secret1h8l5jsd32j2pnr2njluldxxnpwmlftw8w8xtv6",
    "secret16gxdwa79rsvkjaawpcpswthgahm56p4mq28n67",
    "secret16dzd7y7j2yrvlrjqsu54l845rlfw3fevsnqu2y",
    "secret1cx2qm6ep8z2ul030w99c8futt2awg7j79jmkz3",
    "secret1qa3sqtxwhrxa86yqpvwtg0jyp0j4x2pqeakwxh",
    "secret1cr87jhfe2slctt23xyzk7m5yyqxnyr7xq4cx99",
    "secret1makrf3ma4a8zl6qm095utd58xf3h0kdjguzm09",
    "secret1ctnty45uxyclkk2y5uc7y4sgjq6dfd682ur84m",
    "secret1gtgqapev7egfn099yj5tmz65206fjhkp72qae2",
    "secret1anhectwd7fskh49sku2umkad84gu69ykqravcu",
    "secret1x5v2w596cw8uf0vmsaark9xmdnlsfuyy6hz8xx",
    "secret1q6n24vn405k25ujdjlkffd5pyeuypn2mvcue35",
    "secret16mtjs003kntc4kzlqm7yxc9hg2t0gepcntnalg",
    "secret1xedkd7u28vxk2dqgwnvg67xhady0aj8edk2an7",
    "secret1h9m6yn8xtt79gznh2625f2gz5zah5g7jq6hf30",
    "secret12n97vcsv5whd4d4xgdrlrxwk84yagfwd9akgka",
    "secret18jnnkn7sg0g0qyj307ahk5udq838rsldx6eh9w",
    "secret1kh8e0yr2z3ek2ux5vf0pkwp50tlurh0s3prcsz",
    "secret1xv0vl984ntcyvxlj4errhcnuxcjrcm8wfdplh7",
    "secret1w5h4fh48p3x8tkaqt0vdt49vnk082s9vmf5we4",
    "secret1sdvhlj9tehnxpht3d4lm4ch97422ygklxamh4s",
    "secret1uz4c6m3jpdhruwahf4f7q26wwwpmdsvexyyhr8",
    "secret1hn7h3860geka7a0kg7fperqn8u4rdxe2ze9esa",
    "secret1l3lm3lqaet72yv36d4k6c94pw0lw5s4flh3xdt",
    "secret1l3lm3lqaet72yv36d4k6c94pw0lw5s4flh3xdt",
    "secret1dmg852ek58jncpphrlgm6e8rzqlk8qnsl2neg6",
    "secret1np96rzugs3f2auhcu8vhqelst0lphldqaekn9q",
    "secret164mgp63c4y9h2em9j0nfh4q3ju0hdf33lyxnyy",
    "secret14zwg95l409zt3e4j75qnx7ql6m8e3hvse0easg",
    "secret1yhrmaaa8xe8hqthmp8875ewwkwre5dfterxvf3",
    "secret1xywcddj2gf5edmf8hkznm8qyx9t6070jhpgltg",
    "secret1cr87jhfe2slctt23xyzk7m5yyqxnyr7xq4cx99",
    "secret1y2rk820spz4l7ddfkw8r9ewdr5v4w8n0geut3h",
    "secret17qcrkjx45a9qqsg7sn82t85yj3jf32ktcjya5e",
    "secret18yf77mht5wjpjshkfp3c56zs2tk5gezc0z49qr",
    "secret1suwdydkrxegkjrnrnjrjxl3z08320z5kvnkmxa",
    "secret1wcupthd7gz3xe8tn0kx8f6mswnxf6fpsj2qk7e",
    "secret1j7p5eysqdd2t452en6dsl08jgrg4rnjytvt672",
    "secret1f27nfnhcscs6pcwmw99uuhjzspsh60w664q04m",
    "secret1a0ejz2mfrdu6ynrl4vmpx84742a57mnfp2gl0c",
    "secret1a9qq48nayzmq0gvm3vlcz3uvzlrpdwny8n0d7y",
    "secret104jj9mjexjjmuwfaknhxc3vrlnp4v0xl0dupg3",
    "secret1l7sq0pdeq9mh3gfdnthuks7n8cedka5xheh53v",
    "secret1c2pfstte8dz33msnxkn9m0l6096g392uexpk3l",
    "secret1upsegq9l96gpgxg757m444a5rgxpxgfhksmpkx",
    "secret1t4npdrekcregkyqqyrpt02fpmsat3ruclq36g6",
    "secret188het0azy53w9syh8hqaxu72afjd0j9nhf88g2",
    "secret1527uzx6rv25z2xynqm98kfjl6uc3azxk4nxuwu",
    "secret18cc0h4p4vu9m2yamq0srzywk59kfxr8erntmnv",
    "secret1sc7cf4pj5080fthjtk5gw0n770menp4u7yq2z9",
    "secret1r56kcnxm4vknwtmmh4esh9cgwrlxkxf8ekzn5d",
    "secret15cqqsjgjstd6qklts58vt75fu3slkhj9pvad4j",
    "secret1gw0p2s5434a5xfazwc4lumynk42vkzlrtwvjym",
    "secret1flshhasal8vuctm3dussmc8x2lqxyjx65n6zn5",
    "secret1wvgtlxczqrdz6tmc2rg4kcsuu0kazv03yweawx",
    "secret1m6hnwq9aezmyj0u6gxxt0g3edy6fp7lqeczlpt",
    "secret1jsrq84a6zvgf90d4gjxft28cufed6saktu9552",
    "secret1hyn6hnwmuueevz2xfp8jmhaszsvvfuwnd0h5mq",
    "secret13u6wrwgknprmj4djdlqcc0klx4ylwq3d74sdt7",
    "secret1druwflumu0mkx6vg09hpcgwsy52fvfazhn2xmv",
    "secret1vh985nae7q4c2ntfc02775awnu2zkghs4e8zge",
    "secret178tjnfqu3kce5lp0zgkfjexkdsvp3s0ht5h2an",
    "secret1nwl9mluyg290k864qrpuv7aswnwz6szjpw73l0",
    "secret13t9a7kpezmrtn3plsz3pz6nyywcvfyyhuc59th",
    "secret1taqll7kar4pp4w545fzckug9vqtxuqjfra4x67",
    "secret1pvqvzes7f83st3lmvv99mzptalhkl6ug7yc8ht",
    "secret1f4hmea7xdje20zcqtsrll2kzkgrwcmyupefgmr",
    "secret18rnjlztakmsdc2fytyf64r7x0vs8x45pnaszgr",
    "secret19z5k48ky4gws6rv9acl00dxx5rt6pypc35mvnu",
    "secret10gc86pzyrlm9qxpexjccgpnphqgd42umjaqvtz",
    "secret1g3q45wanj7kcvgp0z6tw04qw99slf7s4pndjj2",
    "secret1sdk3q05zzfa6r8qe09jvu90388w9a3zp9gzwrf",
    "secret1l089s3rzq8wd5y5lsz4yhevh3lu9shymdpyap7",
    "secret179pcvf54sfyhgx59nkzz7cshcxxnc6r64es0cw",
    "secret1kdgas29qr0wcyzd2w5tuj654q9pw9ltdxg84f2",
    "secret1gp9hutl33rg4asnpgew3ntyzyrxe9rurjcjy5m",
    "secret1y0jctgapyf2lvsexwku26f9x55klpwvun4p9pc",
    "secret1j8luwgulmkh2y2h9glhljzxhcwnp9lhu05dxxz",
    "secret1j8luwgulmkh2y2h9glhljzxhcwnp9lhu05dxxz",
    "secret1wpaskzrepkgnvdjgw84ftg5tgva2gw6mqzlkly",
    "secret1z5z4tqp7jnruae5zj45udwkrv868rqdr49saa6",
    "secret187pc9753fhdp73z65ljl2nvulnshl0zadndchq",
    "secret1jnjcfvtayl73ma4q6t89hc8tt59832fa8svzne",
    "secret16lm6u0pw3kp9m9us8ttylf9kp6tcls07s63nty",
    "secret18ckfqrt3z9rmznyp8rgygtsx4wqskrddnnntpk",
    "secret1r34n38xeszrkz6squdsacse4ud4gstppkm88ew",
    "secret1hd3vw29nzdpjc63l0z5ndml8hwc5gg50fsnrzw",
    "secret1d698dg6vrgy9hyehdl6zzrk5xyjy44x7qr6wrw",
    "secret10upgq7h2wvuh93aj0ktruckvj03jswtau09h0r",
    "secret1ysccuvn7yl9x3k6lcfqeqmqt5923efdk8d6648",
    "secret1rlhn3rqszeneyu74g7gm5pca2fcjcuq3ampp3m",
    "secret12kktvf80juyrmwqtustzujtf3v3np7c5wptall",
    "secret1lah9uptaewgtad8uy20s5dkvgaxv6ejl3s6dnn",
    "secret1lpeqaljxdxncl4pjvl47kurkylgd7wnr8uhfcs",
    "secret13rk5xksysmh2n6t3u8ujtqx92xfk5w5zjkmluz",
    "secret18x5s49t37y6gave66vllravreh2awtmt5239vt",
    "secret195pkw9fwenm7mjkp8mfg63tjgt4fdwv7g58f8q",
    "secret1q8h3ektlzwuejrrk6yqjstxa4d0hqgpt6qvg0z",
    "secret1ern6g09hca46a4dlmznrz46g0jw38gaywp7w5s",
    "secret16enh7kyjeafs69ugfd09e2xeh443eg6472laj7",
    "secret18gntqyev4edxyxz9l6tpsqtsndcyx9nshnj5zc",
    "secret1yfxwgpme5v0f7gvjuctyxcqpvujvthl3qle9fe",
    "secret1sz53ssfwavvuvkl28yy6740k6puqz4r9l6wj4q",
    "secret17nscd8ze8cvyc74v8dczu9u2ttggu3t6eqma3m",
    "secret13kkgstdnsyl4x3st6lnpcuzxfg9rhwrgyev0n5",
    "secret1ytxkalju65glc8l8y4lzwq86xk9psussqyjva4",
    "secret12k0z284e64l93dhyqhnjdquvzdv6y980wwctgg",
    "secret1njlqxpm8f64g0m6jcazkc4rmcxzaynvfu4gdxz",
    "secret12queq007r5aq3va4qfdwa3mpxyx0hk6pu3st4y",
    "secret1vw6n4w7vy8wd7gynt5953ksupurqxrlr4wfeta",
    "secret14wnx2nnwfc7memxhrz04z0pdfnuvc347efqcu5",
    "secret14anhwkucy0zgc5grxgeeadqsr88vd4ash4kug8",
    "secret1p544h698wyjfjxxzu9e5rwye08afupx6kffasf",
    "secret1m9uf90hagnr0azs88vlfp83twm4cyhkf5t3dgu",
    "secret15ktq4aac20fjwlxmh0avfk52lft34vd44nf0tm",
    "secret1lwhdhju0qptp0r5nvsnx8at0qd3uhx6pnxry85",
    "secret1r8s4vdh55pqj4f5ejra73kt89qw7zwu373aaz0",
    "secret1rfnue0xt0zvhkk8f4scccgeqgss3h2hk94gmcg",
    "secret16aqdn338w02fdjs5t7jxw3jqyq548rj8vp9jda",
    "secret1k48s9fp40dn8w9n500g6zq83cturp0jklh6k3w",
    "secret1cxjhfschqn9sxssq03mr0852stdpwejg8lgj2l",
    "secret1r9l45a585lg6udlne3qx2jeln82lc9lwfg20xw",
    "secret13kncgv2q6fc3lf840xc53jnzvcxena4admljqy",
    "secret1pqt4mahy44rcahqzxa7n7cujn7gwyr0qh5skum",
    "secret186zdfakth9fxvkka3xspwhv0hzqgxvfrzq9yac",
    "secret1qzwk5ntc2vx23gqtjjgg79pg3ykevknnuhcex8",
    "secret1vt880fnmz3sp9sngyjvxcuaf730f6hax7ffxd8",
    "secret1uaat30zvmpzu6rrsjsalgltx65hma2pk9tx8gw",
    "secret1mdc56kecgk47863ry2a79kh76yv6zfp0lv9gfn",
    "secret17y7ag3nlds82njehgdw3thrvm6jx9au5xz5728",
    "secret17mw03kdh7xqt47mz4f07yz0w37ywa6m2z7cuae",
    "secret1fr5nmce4qqrsr9585zveyun857xc58pnvpv5jd",
    "secret1c4gsh6vhzkzreczj7jjn47tt8ym4q5aczalgzl",
    "secret1t3ynwwzdtqukfe9cr940su3v6mjvv0t94lndku",
    "secret1g9ly6w27yn994xhlyjs3svv74updtaxkuqldq6",
    "secret1p6tqd3dp8ac6a0wzde83y9q27shyzyd3ah3xh6",
    "secret1yza5mzgmypm43mzzgwyg3nt958vchxracj3mx3",
    "secret1apt8y47qr57nn0yrg3f6x8accpv7nh2yuy56vs",
    "secret1dg4rsvyja56dlclccy9qa64842zvxd4qpu8jdd",
    "secret1n8uwjfysnd6ehs4wylusmeq52485w2ps3k92f8",
    "secret1xlkzlzqnypyzf3933709wjaya06cpjdpylne90",
    "secret1mlm5n52qsku7pl7cp6nghp5xck6eyf3lxfxljh",
    "secret1mr5jkqa3h74he0hv6wqfruwx7zg9pcq2q73gfm",
    "secret1q40pdghlvpaew8je5jrnnj6ekzrgajtj8ljg04",
    "secret15sjr22c7qelrw4209ay86f8uq2qrq3my8lfjx4",
    "secret16zxx8q898v8ff7wthvs3za3sw3u04wahgcfpld",
    "secret1yjxpjtyxn66pcu0e6w8hehquv5gkju947k02ll",
    "secret1p59zkw2sphq78rkw43l7pn3yv5egv9q9l4wl6j",
    "secret138sjrjw94dgrwyq88qp342fsxm393t6atq077s",
    "secret1gff20xhks9ty8xkwknsa5my6v8u4d25d7lpydy",
    "secret1jl3n7jt34jd4x5v7n7rqn5zela26ulew7pqfef",
    "secret1wlfs5pfkq92cym5a06s0xqd8gnjyds4fdn4d33",
    "secret1dsxe63cktekkus6dz0g9cna7fm64u25wkeuc07",
    "secret1ckw3e8ayl2r27vjclzgster04xvl9vvp28gmrw",
    "secret1vssuqkmqzrkksx0hztqqkf57lwhwexhdc4jfg3",
    "secret1q7nhd8v3cphk4hev25ud9kqkm0jzjcdnvt205a",
    "secret1vtkezj6xmdsvtrg9zkxauwrdyr0qsee9mfcav6",
    "secret16vf6f9zs3umjarfgwsc7dq00rwy6etlqpug56p",
    "secret1msmknmc7w95k6vhp2vrzer77c4rus0yac9nkt2",
    "secret1jpuxu9yfut9dfusfd2slvkep8c6cfdal9j3rf5",
    "secret10u2kwt4hkzuam8yzweqwr7jq8sz2sxy8tncutu",
    "secret13ltsp79gn4enhxs8vdkazq8xh5qjszfcy4fvkh",
    "secret1e82zz0p5h0ndqe9m0rtw5z62d276hfssl9kvuz",
    "secret1g3g7q3mjpsauhvtzmzerx7auf7ltxs4f7z9zq2",
    "secret13pd0m7nkgum0qtg5maggxytkg6k20mgrmerv42",
    "secret15y6unxcvj8tauhaluk8902x6huam0386zxd6x9",
    "secret1qutk4euasvzyc3t7v7w3vxq2jyvx8yuem9a9c4",
    "secret1dcw7ujpq0mxyzaw0u5ny8z92al8664gve88n9m",
    "secret1tjh8yc4rdt47gdracvvnpppaxjzmc96y9wesnn",
    "secret18hup07njxg8p7dm2vxtphq4czq509pmdnkpsfv",
    "secret1nm28hkc5cm7m0f8yc0raxt2zecs3w2p0lp5q0z",
    "secret18qw9ydpewh405w4lvmuhlg9gtaep79vy7lvee2",
    "secret1gg8wk8manj6h9qmuz2nu2ym00x803u49v7vgaw",
    "secret17n40lcz7xu9ce3px3shahqkm8um9p90tdn5rfw",
    "secret1dxs99nkt0nfq2k0j2k7tcwwkku42dr2hhrtp7n",
    "secret123h7kafw9e3lmt9xgvsj67g33jtftycs8uha33",
    "secret146fxqkz500rgas6xr2vlc0s7h4dhkl6w0yfuff",
    "secret1zjdjtuu9fvfarmph2hacxgn87qg3dudwfqmrmd",
    "secret16dr6jyzujfk33wfparlgadxq5c2a08nc2zk96v",
    "secret1vfh72k8p556hq9vhv7am6luxuj0mqscm6xx2fx",
    "secret1zququ7p9n0c9p3ejtz8jj6cs99xdh2nzmxhaul",
    "secret12vez407y7mv0gprarx0gnms0vzktm5n49eg6qq",
    "secret194y4el3cghthf080lflj3tv9m98xvx5p58ny8x",
    "secret1lhe6g9qesttslclf2608aqyqnelqh89q8x5rlg",
    "secret12j7v2v4c3hv4kmfg5r9zwzsnwj8q4vc2xgfdla",
    "secret1jly5hcsv3vwsc5rlc8w53ndeya9rsy8cgzdjja",
    "secret1x3ekx67nxaqeseurpfk8p6txv7v5l5r6c7q8z8",
    "secret1eeru9lncheuv3m8ekc8pntshzna77wtxsfj4xu",
    "secret1peauhgrd5jnnzl6rlfwc74vark6gzyc7fn24cd",
    "secret170xfkavrm6zgcl4cend9jdx90lpwpxejhj9xl4",
    "secret1zv7p7jv5nd0g6yw5sxefqg92lu8ah39vkruzda",
    "secret1ctqpkfjfhtl8vhz52rmf39gzcdfpamftr9h2yh",
    "secret1mlf2cgzlunmmmgd8rxq583hl0k9ckw5wmwyhl2",
    "secret1f0jqkvm47aahj25r8esfsn56j4tz6fhqppl785",
    "secret1766gnpc0xy3juuzr6xkjvdq4lk2g4ed0l0agas",
    "secret1f2snvw50z5698v22t5tnhnrwkld0wjjydruc6k",
    "secret1k7mtfruvmcn98smelywudutr7ty3hctd37sf45",
    "secret1lyfkemg0qtr5wfwml6f2huell3l0f3duapceqy",
    "secret1vph7gxmw33ucsvrethwgwthsgpc5nzqxj5a6xc"
];

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {

    let mut whitelist: Vec<CanonicalAddr> = Vec::default();
    for s in WHITELISTED_ADDRESSES {
        let addr = deps.api.canonical_address(&HumanAddr(s.to_string()))?;
        whitelist.push(addr);
    }

    //Create state
    let state = State {
        contract_owner: deps.api.canonical_address(&env.message.sender).unwrap(),
        seed: msg.seed.as_bytes().to_vec(),
        entropy: Vec::default(),
        start_time: env.block.time,
        winners: Vec::default(),
        whitelist
    };

    // Save to state
    config(&mut deps.storage).save(&state)?;

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::EndLottery {  } => end_lottery(deps, env),
        // HandleMsg::Join { phrase } => {register(deps, env, phrase)},
        HandleMsg::AddToWhitelist { addresses } => {add_to_whitelist(deps, env, addresses)}
    }
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Joined { address } => query_registered(deps, address),
        QueryMsg::Winner {} => query_winner(deps),
        // QueryMsg::Whitelisted {address} => query_whitelist(deps, address),
    }
}

fn throw_gen_err(msg: String) -> StdError {
    StdError::GenericErr {
        msg,
        backtrace: None,
    }
}

//
// fn register<S: Storage, A: Api, Q: Querier>(
//     deps: &mut Extern<S, A, Q>,
//     env: Env,
//     phrase: String
// ) -> StdResult<HandleResponse> {
//     let mut state = config(&mut deps.storage).load()?;
//
//     if !state.whitelist.contains(&env.message.sender) {
//         return Err(throw_gen_err(format!("Address {} is not whitelisted. You may request this address to be added by asking on the phase-2-testnet rocket.chat channel", deps.api.human_address(&env.message.sender)?) ));
//     }
//
//     if state.items.contains(&env.message.sender) {
//         return Err(throw_gen_err(format!("Address {} is already registered", deps.api.human_address(&env.message.sender)?) ));
//     }
//
//     state.items.push(env.message.sender.clone());
//     state.entropy.extend(phrase.as_bytes());
//     state.entropy.extend(env.message.sender.as_slice().to_vec());
//     state.entropy.extend(env.block.chain_id.as_bytes().to_vec());
//     state.entropy.extend(&env.block.height.to_be_bytes());
//     state.entropy.extend(&env.block.time.to_be_bytes());
//
//     state.entropy = Sha256::digest(&state.entropy).as_slice().to_vec();
//
//     // Save state
//     config(&mut deps.storage).save(&state)?;
//
//     Ok(HandleResponse {
//         messages: vec![],
//         log: vec![],
//         data: Some(Binary(Vec::from("Registered successfully!")))
//     })
// }

fn query_registered<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    address: HumanAddr,
) -> StdResult<QueryResponse> {
    let state = config_read(&deps.storage).load()?;

    let addr = deps.api.canonical_address(&address)?;

    if state.whitelist.contains(&addr) {
        Ok(Binary(Vec::from(format!("{} is registered", address))))
    } else {
        Ok(Binary(Vec::from(format!("{} is not registered", address))))
    }
}

fn query_winner<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
) -> StdResult<QueryResponse> {
    let state = config_read(&deps.storage).load()?;

    if state.winners.len() < 10 {
        return Ok(Binary(Vec::from(format!("Winners not selected yet!"))));
    }

    let mut winners_readable: Vec<String> = Vec::default();
    for winner in state.winners.iter() {
        winners_readable.push(deps.api.human_address(winner).unwrap().to_string())
    }

    Ok(Binary(Vec::from(format!("Winners: {:?}", winners_readable))))
}

fn add_to_whitelist<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    mut addresses: Vec<HumanAddr>,
) -> StdResult<HandleResponse> {
    // TODO Check if contract has expired

    let mut state = config(&mut deps.storage).load()?;

    if deps.api.canonical_address(&env.message.sender).unwrap() != state.contract_owner {
        return Err(throw_gen_err("You cannot trigger lottery end unless you're the owner!".to_string()));
    }

    let i = addresses.iter_mut();
    for x in i {
        state.whitelist.push(deps.api.canonical_address(x)?)
    }

    config(&mut deps.storage).save(&state)?;

    Ok(HandleResponse::default())
}

fn end_lottery<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> StdResult<HandleResponse> {
    // TODO Check if contract has expired

    let mut state = config(&mut deps.storage).load()?;

    if deps.api.canonical_address(&env.message.sender).unwrap() != state.contract_owner {
        return Err(throw_gen_err("You cannot trigger lottery end unless you're the owner!".to_string()));
    }
    // let contract_addr: HumanAddr = deps.api.human_address(&env.contract.address)?;

    // this way every time we call the end_lottery function we will get a different result. Plus it's going to be pretty hard to
    // predict the exact time of the block, so less chance of cheating
    state.entropy.extend_from_slice(&env.block.time.to_be_bytes());

    let mut rng: Prng = Prng::new(&state.seed, &state.entropy);

    let mut winners: Vec<CanonicalAddr> = Vec::default();

    while winners.len() < 10 {
        let winner = rng.select_one_of(state.whitelist.clone().into_iter());

        if winner.is_none() {
            return Err(throw_gen_err(format!("Fucking address is empty wtf")));
        }
        let unwrapped = winner.unwrap();

        if !winners.contains(&unwrapped) {
            winners.push(unwrapped);
        }
    }

    let mut winners_readable: Vec<String> = Vec::default();
    for winner in winners.iter() {
        winners_readable.push(deps.api.human_address(winner).unwrap().to_string())
    }

    state.winners = winners;

    config(&mut deps.storage).save(&state)?;


    Ok(HandleResponse {
        messages: vec![],
        log: vec![log("winners", format!("{:?}", winners_readable))],
        data: None,
    })
}
