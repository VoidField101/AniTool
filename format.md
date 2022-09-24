# ANI Format

ANI files are encoded with the RIFF format with a ``ACON`` header ID.


There are 2 types of chunks:
- ``RIFF`` and ``LIST`` which contain other chunks and are composed of: 
	- their name either ``RIFF`` or ``LIST`` \[4 bytes\]
	- length \[4 bytes\]
	- the header id e.g. ``ACON`` (for Riff) and ``fram`` (for list) \[4 bytes\]
	- sub chunks with the length determined by the length field (Note since the header id is part of the length the total length of sub chunks is: length - 4)
- ``anih``, ``seq``, ``rate`` & ``icon`` which don't contain any other chunks and don't contain a header id. They consist of:
	- their name \[4 bytes\]
	- length \[4 bytes\]
	- data with the length determined by the length field


Lengths are stored in **little endian** unsigned 32bit integers. This is generally the case for most if not all numbers in this format.

Structure for containers (``RIFF`` and ``LIST``):
```c
{
	char[4]             name; // RIFF or LIST
	uint32_t            length; // of header_id + content
	char[4]             header_id; //ACON or fram
	char[length - 4]    content;
}
```

Structure for chunks:
```c
{
	char[4]             name; // anih, seq, rate or icon
	uint32_t            length; // of content
	char[length]        content;
}
```
**NOTE**: "chunk" and "container" are terms I use. They may not be the official terms.

Structure of containers and chunks:
```
'RIFF' (ACON)
	|
	+-- 'anih'
	|
	+-- 'seq ' (optional)
	|
	+-- 'rate' (optional)
	|
	+-- 'LIST' (fram)
			|
			+-- 'icon'
			|
			...
```

**NOTE**: ``anih``, ``seq ``, ``rate`` & ``LIST`` can be in any random order. Typically ``anih`` is the first element.



## RIFF
The file starts with the ``RIFF`` header which contains the length of the file and ``ACON`` as header id. This also means that the entire file is a child element of the ``RIFF`` chunk. 

The ``ACON`` header id hints to the fact that this is a animated cursor file (Riff can be used for other purposes). Therefor ``ACON`` is required in this context.

## LIST
Contains a list of multiple repeating elements. The ``fram`` header id means that this list contains a list of frames used in this animation. 

One important note for sequencing is that elements are indexed in the order they are stored (so it's important to keep track of the order). Indices start at 0.

## anih
This chunk contains information about the animated cursor. It is typically the first header after the ``RIFF`` header but it does not have to be the case.

```c
{
	uint32_t        length; // of entire struct (always 36) {1}
	uint32_t        frames; // number of frames
	uint32_t		steps; // number of steps {2}
	uint32_t		width; // width of the cursor {3}
	uint32_t		height; // height of the cursor {3}
	uint32_t		bits; // number of bits per pixel {3}
	uint32_t		planes; // number of planes (=1) {3}
	uint32_t		rate; // default time per frame (in 1/60s steps) {4}
	uint32_t		flags;
}
```

**NOTE 1:**	This length is separately stored from the length in the header. Therefor in the file the number 36 (0x24) will be stored twice.

**NOTE 2** The sequence can be longer than the number of frames. This can only be the case when a ``seq `` header is present.

**NOTE 3** These fields are only used when raw bitmap icons are used. This rarely seems to be the case in reality.

**NOTE 4** The rate is only used when no ``rate`` header is present. It is ignored otherwise.

There are only 2 bits in the flags who are used:
- Bit 0: Icons are encoded as ``ICO`` or ``CUR`` when set and as raw bitmap data otherwise.
- Bit 1: When set this indicates that a ``seq `` chunk is present.

## seq (Optional)
Defines the sequence of frames for the animation. This chunk should only be present when the sequence bit in anih is set.

Each entry is a 32-bit index referencing the frame in the "fram-LIST"

The number of entries depends on ``anih.steps`` and the total size is ``anih.steps * 4``. (total size is also stored in the generic chunk header)

When this chunk is present the animation can be longer than the number of frames since frames can be used multiple times in a sequence. The default sequence when the header is not present is the order which the icons are stored.

**NOTE:** The since the header name always uses 4 bytes there is a space at the end: ``'seq '``

## rate (Optional)
This chunk overrides the default animation time in the anih chunk.

The number of entries depends on ``anih.steps`` and the total size is ``anih.steps * 4``. (total size is also stored in the generic chunk header)

Each entry is a 32-bit number defining the time the frame is shown in 1/60s steps. 

When this header is missing every frame has the same timing defined by the ``anih.rate`` value.

## icon
This chunk contains the icon information and it can only appear in the frame list. 

The data depends on the icon flag in anih. If the flag is set the data is either encoded as a ``ICO`` or ``CUR`` file, the distinction between those is possible with their respective headers.
When the bit is not set the data is stored in a raw bitmap with the width, height, bitdepth, etc. defined in the anih chunk.

