// This file contains a model of a reflector. The reflector is a strip 
// placed into the modelling space, then moved in dip or direction.It 
// is defined by its length on dip, its width, in strike direction, and 
// a vector of offsets from the dip line.

// The reflector is made up of a number of planes. Each one is defined 
// by three points, and is rectangular in plan view. The reflector can 
// be iterated over planes, or that's the plan.



//Cross product of two vectors gives a normal to the two vectors!


struct 