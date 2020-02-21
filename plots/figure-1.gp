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
set border 15 front linecolor rgb "black"  linewidth 1.000 dashtype solid
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
set format x "%g" 
set format y "%g" 
set format x2 "" 
set format y2 "" 
set format z "%g" 
set format cb "%g" 
set format r "%g" 
set ttics format "% h"
set timefmt "%d/%m/%y,%H:%M"
set angles radians
set tics back
set grid nopolar
set grid xtics nomxtics ytics nomytics noztics nomztics nortics nomrtics \
 nox2tics nomx2tics noy2tics nomy2tics nocbtics nomcbtics
set grid layerdefault   lt 0 linecolor 0 linewidth 0.500,  lt 0 linecolor 0 linewidth 0.500
unset raxis
set theta counterclockwise right
set style parallel front  lt black linewidth 2.000 dashtype solid
set key title "" center
set key fixed right top vertical Right noreverse enhanced autotitle box lt black linewidth 1.000 dashtype solid
set key noinvert samplen 4 spacing 1 width 0 height 0 
set key maxcolumns 0 maxrows 0
set key opaque
unset key
unset label
unset arrow
set style increment default
unset style line
set style line 101  linecolor rgb "#808080"  linewidth 1.000 dashtype solid pointtype 1 pointsize default
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
set origin 0,0
set style data lines
set style function lines
unset xzeroaxis
unset yzeroaxis
unset zzeroaxis
unset x2zeroaxis
unset y2zeroaxis
set xyplane relative 0
set tics scale  1, 0.5, 1, 1, 1
set mxtics default
set mytics default
set mztics default
set mx2tics default
set my2tics default
set mcbtics default
set mrtics default
set nomttics
set xtics border out scale 0.75,0.375 nomirror norotate  autojustify
set xtics  norangelimit autofreq 
set ytics border out scale 0.75,0.375 nomirror norotate  autojustify
set ytics  norangelimit autofreq 
set ztics border out scale 0.75,0.375 nomirror norotate  autojustify
set ztics  norangelimit autofreq 
set x2tics border in scale 0.75,0.375 nomirror norotate  autojustify
set x2tics  norangelimit autofreq 
set y2tics border in scale 0.75,0.375 nomirror norotate  autojustify
set y2tics  norangelimit autofreq 
set cbtics border out scale 0.75,0.375 nomirror norotate  autojustify
set rtics axis out scale 0.75,0.375 nomirror norotate  autojustify
set rtics  norangelimit autofreq 
unset ttics
set title "" 
set title  font "" textcolor lt -1 norotate
set timestamp bottom 
set timestamp "" 
set timestamp  font "" textcolor lt -1 norotate
set trange [ * : * ] noreverse nowriteback
set autoscale tfixmin
set autoscale tfixmax
set urange [ * : * ] noreverse nowriteback
set autoscale ufixmin
set autoscale ufixmax
set vrange [ * : * ] noreverse nowriteback
set autoscale vfixmin
set autoscale vfixmax
set xlabel  font "" textcolor lt -1 norotate
set x2label "" 
set x2label  font "" textcolor lt -1 norotate
set xrange [ 0.00000 : 2048.00 ] noreverse writeback
set autoscale xfixmin
set autoscale xfixmax
set x2range [ * : * ] noreverse writeback
set autoscale x2fixmin
set autoscale x2fixmax
set ylabel "pi(+1|theta)" 
set ylabel  font "" textcolor lt -1 rotate
set y2label "" 
set y2label  font "" textcolor lt -1 rotate
set yrange [ 0.00000 : 1.00000 ] noreverse writeback
set autoscale yfixmin
set autoscale yfixmax
set y2range [ * : * ] noreverse writeback
set autoscale y2fixmin
set autoscale y2fixmax
set zlabel "" 
set zlabel  font "" textcolor lt -1 norotate
set zrange [ * : * ] noreverse writeback
set autoscale zfixmin
set autoscale zfixmax
set cblabel "log(probability density)" 
set cblabel  font "" textcolor lt -1 rotate
set cbrange [ 0.000100000 : 1.00000 ] noreverse writeback
set autoscale cbfixmin
set autoscale cbfixmax
set rlabel "" 
set rlabel  font "" textcolor lt -1 norotate
set rrange [ * : * ] noreverse writeback
set autoscale rfixmin
set autoscale rfixmax
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
set colorbox default
set colorbox vertical origin screen 0.9, 0.2 size screen 0.05, 0.6 front  noinvert bdefault
set style boxplot candles range  1.50 outliers pt 7 separation 1 labels auto unsorted
set loadpath "/home/cc/.local/lib/gnuplot" 
set fontpath 
set psdir
set fit brief errorvariables nocovariancevariables errorscaling prescale nowrap v5

load '../viridis.pal'

tm = 0.95
bm = 0.15
mp = 0.05

lm = 0.125
rm = 0.75

set lmargin at screen lm
set rmargin at screen rm

set yrange [0:1]
set ylabel '$p_+$' offset 1.25,0
set ytics format "%.1f" offset 0.5

set xrange [0:800]
set xtics 0, 200, 800
set x2tics 0, 200, 800

set cbrange [1e-5:1]
set logscale cb
set cbtics in format "$10^{%L}$" offset -0.5
set cblabel '$\Pr{\pi(a = +1\mid\theta_t) = p_+}$'

set tmargin at screen tm
set bmargin at screen ((tm + bm + mp) / 2)
unset colorbox

lx = 550
ly = 0.6

set term cairolatex pdf size 3.25in,3.25in
set output 'transitions-plot.tex'

set multiplot layout 2,1;

set xtics format ""
unset xlabel
plot 'fig1-time-evo-0.50.bin' binary matrix u 2:($1/1024.0):3 w image

set colorbox user origin (rm + mp*0.75),bm size 0.05,(tm-bm) 
set tmargin at screen ((tm + bm - mp) / 2)
set bmargin at screen bm

set xtics format "%g" offset 0,0.25
set xlabel "$t$ (SGA Iteration)" 
plot 'fig1-time-evo-0.56.bin' binary matrix u 2:($1/1024.0):3 w image

unset multiplot
unset output
