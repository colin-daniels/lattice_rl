set border 15 front linecolor rgb "black" linewidth 1.000 dashtype solid
set grid xtics nomxtics ytics nomytics noztics nomztics nortics nomrtics nox2tics nomx2tics noy2tics nomy2tics nocbtics nomcbtics
unset key
unset label
unset arrow
unset object
set style line 101  linecolor rgb "black"  linewidth 1.000 dashtype solid pointtype 1 pointsize default
set style data lines
set style function lines
set xtics border out scale 0.75,0.375 nomirror norotate  autojustify
set xtics  norangelimit autofreq 
set ytics border out scale 0.75,0.375 nomirror norotate  autojustify
set ytics  norangelimit autofreq 
unset x2tics
unset y2tics
set cbtics border in scale 0.75,0.375 mirror norotate  offset character 0, 0.25, 0 autojustify
set cbtics  norangelimit autofreq
set palette positive nops_allcF maxcolors 0 gamma 1.5 color model RGB

load '../moreland.pal'
set palette negative

set xlabel  font "" textcolor lt -1 norotate
set ylabel  font "" textcolor lt -1 rotate

set cblabel "" 
set cblabel  font "" textcolor lt -1 rotate

set x2tics border in format "" scale 0.75,0.375 nomirror norotate  autojustify
set y2tics border in format "" scale 0.75,0.375 nomirror norotate  autojustify

lm = 0.15
rm = 0.99
bm = 0.12
tm = 0.98

cb_y = 0.05
cb_h = 0.04

cb_x = lm
cb_w = rm - lm

set lmargin at screen lm
set rmargin at screen rm

set cbrange [1:8]
set xrange [0:1]
set xtics 0, 0.25, 1.0
set x2tics 0, 0.25, 1.0 format ""

unset grid
unset xlabel
unset object

set term cairolatex pdf size 3.25in,4in; set output '1d-value-grad.tex'
set multiplot layout 2,1

set yrange [-22.5:12.5]
set ytics -20, 5, 10
set y2tics -20, 5, 10 format ""
set ylabel "$v_{\\pi,9,9}(s)$" offset 1,0

set tmargin at screen tm
set bmargin at screen ((tm - bm) / 2 + bm)
unset xtics
unset colorbox

set border 14
plot 0 lc rgb '#444444' dt '_', 'value-fn-1d.dat' u 1:2:4 lw 3 lc palette dt solid

set border 15
unset colorbox

set tics front
set xlabel "$p := \\pi(a = +1)$" offset 0,0.5
set xtics out nomirror offset 0,0.325
set yrange [-80:135] noreverse writeback
set ytics -60, 30, 120
set y2tics -60, 30, 120 format ""
set ylabel "$\\partial_{p}\\; v_{\\pi,9,9}(s)$" offset 1,0

set tmargin at screen ((tm - bm) / 2 + bm)
set bmargin at screen bm

set obj 4 rect from 0, -80 to 1,0 fc rgb '#DDDDDD' fs pattern 6 noborder back

LABEL="$\\partial_p v_\\pi \\leq 0$"
set obj 10 rect at 0.75,-30 size char strlen("P V <= 0"), char 1.5 fc rgb "white"
set label 10 LABEL at 0.75,-30 front center

plot 0 lc rgb '#444444' dt '_', 'value-fn-1d.dat' u 1:3:4 lw 3 lc palette dt solid
unset multiplot
unset output

