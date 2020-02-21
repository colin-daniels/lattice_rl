#!/usr/bin/gnuplot -persist
#
#    
#    	G N U P L O T
#    	Version 5.2 patchlevel 8    last modified 2019-12-01 
#    
#    	Copyright (C) 1986-1993, 1998, 2004, 2007-2019
#    	Thomas Williams, Colin Kelley and many others
#    
#    	gnuplot home:     http://www.gnuplot.info
#    	faq, bugs, etc:   type "help FAQ"
#    	immediate help:   type "help"  (plot window: hit 'h')
# set terminal qt 0 font "Sans,9"
# set output
unset clip points
set clip one
unset clip two
set errorbars front 1.000000 
set border 31 front lt black linewidth 1.000 dashtype solid
set zdata 
set ydata 
set xdata 
set y2data 
set x2data 
set boxwidth
set style fill  empty border
set style rectangle back fc  bgnd fillstyle   solid 1.00 border lt -1
set style circle radius graph 0.02 
set style ellipse size graph 0.05, 0.03 angle 0 units xy
set dummy x, y
set format x "% h" 
set format y "% h" 
set format x2 "% h" 
set format y2 "% h" 
set format z "% h" 
set format cb "% h" 
set format r "% h" 
set ttics format "% h"
set timefmt "%d/%m/%y,%H:%M"
set angles radians
set tics back
unset grid
unset raxis
set theta counterclockwise right
set style parallel front  lt black linewidth 2.000 dashtype solid
set key title "" center
set key fixed right top vertical Right noreverse enhanced autotitle nobox
set key noinvert samplen 4 spacing 1 width 0 height 0 
set key maxcolumns 0 maxrows 0
set key noopaque
unset key
unset label
unset arrow
set style increment default
unset style line
unset style arrow
set style histogram clustered gap 2 title textcolor lt -1
unset object
set style textbox transparent margins  1.0,  1.0 border  lt -1 linewidth  1.0
set offsets 0, 0, 0, 0
set pointsize 1
set pointintervalbox 1
set encoding default
unset polar
unset parametric
unset decimalsign
unset micro
unset minussign
set view 60, 30, 1, 1
set view azimuth 0
set rgbmax 255
set samples 100, 100
set isosamples 10, 10
set surface 
unset contour
set cntrlabel  format '%8.3g' font '' start 5 interval 20
set mapping cartesian
set datafile separator whitespace
unset hidden3d
set cntrparam order 4
set cntrparam linear
set cntrparam levels 5
set cntrparam levels auto
set cntrparam firstlinetype 0 unsorted
set cntrparam points 5
set size ratio -1 1,1
set origin 0,0
set style data points
set style function lines
unset xzeroaxis
unset yzeroaxis
unset zzeroaxis
unset x2zeroaxis
unset y2zeroaxis
set xyplane relative 0.5
set tics scale  1, 0.5, 1, 1, 1
set mxtics default
set mytics default
set mztics default
set mx2tics default
set my2tics default
set mcbtics default
set mrtics default
set nomttics
set xtics border in scale 1,0.5 mirror norotate  autojustify
set xtics  norangelimit autofreq 
set ytics border in scale 1,0.5 mirror norotate  autojustify
set ytics  norangelimit autofreq 
set ztics border in scale 1,0.5 nomirror norotate  autojustify
set ztics  norangelimit autofreq 
unset x2tics
unset y2tics
set cbtics border in scale 1,0.5 mirror norotate  autojustify
set cbtics  norangelimit autofreq 
set rtics axis in scale 1,0.5 nomirror norotate  autojustify
set rtics  norangelimit autofreq 
unset ttics
set title "" 
set title  font "" textcolor lt -1 norotate
set timestamp bottom 
set timestamp "" 
set timestamp  font "" textcolor lt -1 norotate
set trange [ * : * ] noreverse nowriteback
set urange [ * : * ] noreverse nowriteback
set vrange [ * : * ] noreverse nowriteback
set xlabel "" 
set xlabel  font "" textcolor lt -1 norotate
set x2label "" 
set x2label  font "" textcolor lt -1 norotate
set xrange [ 0.00000 : 1.00000 ] noreverse writeback
set x2range [ * : * ] noreverse writeback
set ylabel "" 
set ylabel  font "" textcolor lt -1 rotate
set y2label "" 
set y2label  font "" textcolor lt -1 rotate
set yrange [ 0.00000 : 0.866025 ] noreverse writeback
set y2range [ * : * ] noreverse writeback
set zlabel "" 
set zlabel  font "" textcolor lt -1 norotate
set zrange [ * : * ] noreverse writeback
set cblabel "" 
set cblabel  font "" textcolor lt -1 rotate
set cbrange [ -8.00000 : 5.00000 ] noreverse writeback
set rlabel "" 
set rlabel  font "" textcolor lt -1 norotate
set rrange [ * : * ] noreverse writeback
unset logscale
unset jitter
set zero 1e-08
set lmargin  -1
set bmargin  -1
set rmargin  -1
set tmargin  -1
set locale "en_US.UTF-8"
set pm3d explicit at s
set pm3d scansautomatic
set pm3d interpolate 1,1 flush begin noftriangles noborder corners2color mean
set pm3d nolighting
set palette positive nops_allcF maxcolors 0 gamma 1.5 color model RGB 
set palette rgbformulae 7, 5, 15
set colorbox default
set colorbox vertical origin screen 0.9, 0.2 size screen 0.05, 0.6 front  noinvert bdefault
set style boxplot candles range  1.50 outliers pt 7 separation 1 labels auto unsorted
set loadpath 
set fontpath 
set psdir
set fit brief errorvariables nocovariancevariables errorscaling prescale nowrap v5
GNUTERM = "qt"
## Last datafile plotted: "value-3d-(1-4-0).txt"
set yrange [0:1]
set xrange [0:1]

set cntrparam levels incr -5,1,5
set contour
load '../viridis.pal'
set border lw 2
unset border
unset tics



apex_x = sqrt(3)/2
apex_y = 0.5

set object 1 polygon from 0,0 to apex_x,apex_y to 0,1 to 0,0
set object 1 fillstyle empty border lw 2 lc rgb 'black' front

obj_n = 2
ts = 8.0
divs = 5.0
# 
# # set label "\\large $y$" at apex_x/2,apex_y/2 right offset -6,1.75
# # set label "\\large $x$" at 1/2.0,0 center offset 0,-3
# # set label "\\large $z$" at (1 - apex_x/2), apex_y/2 left offset 6,1.75
# 

dx = apex_x / divs 
dy = apex_y / divs
dxt = dy / ts
dyt = dx / ts
d1t = 1.0 / (ts * divs)


# set label "\\large $p_x$" at apex_x/2,apex_y/2 right offset 4,-1.75
# set label "\\large $p_y$" at apex_x/2, (1 - apex_y/2) left offset 2.3,2.1
# set label "\\large $p_z$" at 0,0.5 center offset -5,0.5

set label "$p_x$" at apex_x/2,apex_y/2 right offset 3,-1.75
set label "$p_y$" at apex_x/2, (1 - apex_y/2) left offset 1.3,2.1
set label "$p_z$" at 0,0.5 center offset -3.5,0.5


do for [i=0:divs] {
    x = dx * i
    y = dy * i
    
    if (i!=divs) {
        set arrow obj_n from 0,(i/divs) to (-dyt),(i/divs+dxt) nohead lc rgb 'black' lw 2 front
        obj_n = obj_n + 1
        set arrow obj_n from x,(1 - y) to (x+dxt),(1 - y+dyt) nohead lc rgb 'black' lw 2 front
        obj_n = obj_n + 1
        
        set label sprintf("%.1f", 1.0 - i / divs) at (-dyt),(i/divs+dxt) right offset -0.2,0.125 front
        set label sprintf("%.1f", 1.0 - i / divs) at (x+dxt),(1 - y+dyt) left offset 0.25,0.25 front
    }
    if (i != 0) {
        set arrow obj_n from x,y to x,(y-d1t) nohead lc rgb 'black' lw 2 front
        obj_n = obj_n + 1
        
        set label sprintf("%.1f", i / divs) at x,(y-d1t) center offset 0.3,-0.7 front
    }
    
    
}
set cbrange [*:*]

set colorbox
set cbtics
# NOTE: q-r-0 is NOT the same as the x/y/z coords in figure 2
set size noratio

bm=0.1
tm=0.9
lm=0.4
rm=0.975
set bmargin at screen bm
set tmargin at screen tm
set lmargin at screen lm
set rmargin at screen rm

cpad = 0.195
cw = 0.055
set cbtics offset -5.1,0 format "%g"
set cbtics scale 0.75,0.5
set cblabel "Value" offset -9.5
set colorbox user origin (lm-cpad - cw),bm size cw,(tm-bm)

set term cairolatex pdf size 2.166in,1.7in
set cbtics -12,2,0
set cbrange [-12:-3]
set output 'value-3d-3-1-0.tex'

set label 20 "\\large \\textbf{(b)}" at screen 0.01,0.95 left
plot 'value-3d-(2-1).txt' w image, 'value-3d-(2-1).txt.tab' w lines lc rgb 'white'
set output

set term cairolatex pdf size 2.166in,1.7in
set cbtics -12,1,10
set cbrange [-12:-9.5]
set output 'value-3d-4-2-0.tex'
set label 20 "\\large \\textbf{(a)}" at screen 0.01,0.95 left
# FIXME: contours
plot 'value-3d-(2-2).txt' w image, 'value-3d-(2-2).txt.tab' w lines lc rgb 'white'
set output


set term cairolatex pdf size 2.166in,1.7in
set cbtics -10,2,-2
set cbrange [-11:0]
set output 'value-3d-2-1-0.tex'
set label 20 "\\large \\textbf{(c)}" at screen 0.01,0.95 left
plot 'value-3d-(1-1).txt' w image, 'value-3d-(1-1).txt.tab' w lines lc rgb 'white'
set output

# set object 100 polygon from 0,0 to apex_x,apex_y to 0,1 to 0,0
# set object 100 fillstyle solid fc rgb '#440356' back
# plot 'grad-3d-(2-2-0).txt' u ($1-$3):($2-$4):($3*$5/12):($4*$5/12):5 w vectors  lc palette lw 2.5
# plot 'value-3d-(2-2-0).txt' w image, 'grad-3d-(2-2-0).txt' u ($1-$3):($2-$4):($3*$5/12):($4*$5/12):6 w vectors lc rgb 'white' lw 2
 
 
# set multiplot layout 4,4;
# pltit="plot infile w image, infile.'.tab' w lines lc rgb 'white'"
# 
# infile='value-3d-(1-4-0).txt'; @pltit; plot NaN; plot NaN; plot NaN; 
# infile='value-3d-(1-3-0).txt'; @pltit; infile='value-3d-(2-3-0).txt'; @pltit; plot NaN; plot NaN; 
# infile='value-3d-(1-2-0).txt'; @pltit; infile='value-3d-(2-2-0).txt'; @pltit; infile='value-3d-(3-2-0).txt'; @pltit; plot NaN;
# infile='value-3d-(1-1-0).txt'; @pltit; infile='value-3d-(2-1-0).txt'; @pltit; infile='value-3d-(3-1-0).txt'; @pltit; infile='value-3d-(4-1-0).txt'; @pltit;
# unset multiplot


# set output
