set border 15 front linecolor rgb "black" linewidth 1.000 dashtype solid
set grid xtics nomxtics ytics nomytics noztics nomztics nortics nomrtics \
 nox2tics nomx2tics noy2tics nomy2tics nocbtics nomcbtics
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
set x2tics border in format "" scale 0.75,0.375 nomirror norotate  autojustify
set y2tics border in format "" scale 0.75,0.375 nomirror norotate  autojustify

unset key
unset grid
unset object
set border 15
set tics front
load '../moreland.pal'
set palette negative

lm = 0.1
rm = 0.91
tm = 0.95
bm = 0.2
psep = 0.02
pwidth = (rm - lm - 2 * psep) / 3

pratio = pwidth / (tm - bm)

set bmargin at screen bm
set tmargin at screen tm

set cbrange [0:1]
set cblabel "$\\Pr\\{\\pi(+1 \\mid \\theta_{\\infty}) = 1.0\\}$"
set cbtics out nomirror 

set xrange [0:1]
set xtics out nomirror 0, 0.25, 1.0
set x2tics in 0, 0.25, 1.0 format ""
set xlabel "$\\pi(+1 \\mid \\theta_{0})$" offset 0,0.25

set yrange [-5:1]
set ytics out nomirror -5, 1, 1 format "$10^{%g}$"
set y2tics in -5, 1, 1 format ""
set ylabel "Step Size" offset 0,0

set term cairolatex color pdf size 6.75in,(pratio * 6.75)in; 
set output 'convergence-prob-multi.tex'

set multiplot layout 1, 3;
unset colorbox

lx = 0.125 + .125/2
ly = -4.25
set obj 10 rect at lx,ly size char strlen("s0 = 0"), char 1.5 fs empty border lc rgb 'white' front lw 4
set label 10 "\\textcolor{white}{$s_0 = 1$}" at lx,ly front center

set lmargin at screen lm
set rmargin at screen (lm + pwidth)
plot 'fig5-converge-s-1.bin' binary matrix u ($1/1024):(($2/1024.0) * 6 - 5):(1.0 - $3) w image

set label 10 "\\textcolor{white}{$s_0 = 3$}"
unset ylabel
set ytics format ""
set lmargin at screen (lm + pwidth + psep)
set rmargin at screen (lm + 2 * pwidth + psep)
plot 'fig5-converge-s-3.bin' binary matrix u ($1/1024):(($2/1024.0) * 6 - 5):(1.0 - $3) w image

set label 10 "\\textcolor{white}{$s_0 = 5$}"
set colorbox
set lmargin at screen (lm + 2 * pwidth + 2 * psep)
set rmargin at screen rm
plot 'fig5-converge-s-5.bin' binary matrix u ($1/1024):(($2/1024.0) * 6 - 5):(1.0 - $3) w image
unset multiplot

unset output

