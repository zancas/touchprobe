A collection of information relevant to implementing a touch probe extension
to LinuxCNC.

Note:  It's not clear to me, at this point, whether this repo will ever
contain code...  much less Rust code...  but, well...  one can hope!


LinuxCNC
========
https://github.com/LinuxCNC/linuxcnc.git


Plenty Of Room At The Bottom
============================
http://www.zyvex.com/nanotech/feynman.html


Fadedbits
=========
http://fadedbits.com/2011/02/touchprobe/

I like how this project includes adjustment screws to center the probe,
and a connector so you can disconnect the signal wire in order to spin the
probe to center it.  That seems like an important feature.  The location
of the probe is determined by the location of the balls, so centering
the probe is accomplished by centering the part that locates the balls.


Vinland
=======
http://www.vinland.com/Touch-Probe.html

This is a good clean example of a non-adjustable probe with soldered,
glued balls.


Brusselsprout
=============
http://www.brusselsprout.org/CNC/1P-Probe/

This design uses just three balls (instead of six) and a flat disk
connected to the probe (instead of three rods).  I'm not sure how that
affects the behavior of the probe.


Indoor Flyer (defunct)
======================
http://web.archive.org/web/20121126174517/http://www.indoor.flyer.co.uk:80/probe.htm

I really like how this design clamps the balls rather than glue them
into place, because I think the thickness of the glue forces the balls
to be displaced in uncontrolled ways.

I also like how this design uses a conductive substrate to connect the
balls electrically, rather than solder wires to them, just because it
seems simpler to build.  I'm not sure if it's better to use copper clad
board for the substrate like this design does, or just thin aluminum
plate.


Martin Kennedy's article
========================
http://www.homemetalshopclub.org/projects/touch_probe/touch_probe.html

This design uses clamping rather than gluing to hold the balls in place.

I like how it has an LED indicator showing the state of the probe.

The "Drawings" link has excellent mechanical drawings showing the
construction in detail.


Rust
====
https://doc.rust-lang.org/book/second-edition/ch12-03-improving-error-handling-and-modularity.html
