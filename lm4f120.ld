/*
* Copyright (c) 2012, Mauro Scomparin
* Copyright (c) 2017, Jonathan 'theJPster' Pallant
* All rights reserved.
*
* Redistribution and use in source and binary forms, with or without
* modification, are permitted provided that the following conditions are met:
*     * Redistributions of source code must retain the above copyright
*       notice, this list of conditions and the following disclaimer.
*     * Redistributions in binary form must reproduce the above copyright
*       notice, this list of conditions and the following disclaimer in the
*       documentation and/or other materials provided with the distribution.
*     * Neither the name of Mauro Scomparin nor the
*       names of its contributors may be used to endorse or promote products
*       derived from this software without specific prior written permission.
*
* THIS SOFTWARE IS PROVIDED BY THE AUTHOR(S) ``AS IS'' AND ANY
* EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
* WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
* DISCLAIMED. IN NO EVENT SHALL THE AUTHOR(S) BE LIABLE FOR ANY
* DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
* (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
* LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
* ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
* (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
* SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*
* Author:   Mauro Scomparin <http://scompoprojects.wordpress.com>.
* Author:   Jonathan Pallant <http://railwayelectronics.blogspot.com>.
* Description:  Linker description file for LM4FXXX microcontrollers.
*/

/*
* Memory definition:
* FLASH:  start point 0x00,   length 0x10000.
* SRAM:   start point 0x20000000  length 0x8000.
*   VAR:    enough to hold .data and .bss
*   HEAP:   All the remaining space, up to...
*   STACK:  start point 0x20007000  length 0x1000.
*/
MEMORY
{
    /* Limit ourselves to the first 64 KiB of flash, leaving 192 KiB */
    FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 0x00010000
    /* But we can use all the SRAM */
    SRAM (rwx) : ORIGIN = 0x20000000, LENGTH = 0x00008000
}

_stack_size = 0x1000;

/*
* Sections definitions:
*
* .text     - machine instructions.
* .ARM*     - No idea
* .eh_frame - No idea
* .data     - initialized data defined in the program.
* .bss      - un-initialized global and static variables (to be initialized to 0 before starting main).
*/
SECTIONS
{
    /* This section it's the code, containing the NVIC Vector table that must start at 0x0
     *   Look at the LM4F120H5QR datasheet for details. (Table 2-8. Exception Types)
     */

     . = 0x00000000;

    .text :
    {
        KEEP(*(.nvic_table))     /* This must be first. It tells the ARM where the stack and IRQ vectors are. */
                                 /* KEEP stops it being removed as we don't refer to it anywhere. */
        . = ALIGN(1024);         /* Reserve room for flags and attributes at address 0x400 */
        KEEP(*(.attributes .attributes.*))
        *(.text*)                /* This contains most of the executable code. */
        *(.rodata*)              /* Read only data (const variables) */
        . = ALIGN(4);
    } > FLASH

    .ARM.exidx : {
        __exidx_start = .;
        *(.ARM.exidx* .gnu.linkonce.armexidx.*)
        __exidx_end = .;
    } > FLASH

    _data_start_flash = ALIGN(8);

    /*
     * .data segment must be placed in RAM but it's originally stored in FLASH
     * So I set the data segment in ram, but I specify the load address with the AT
     * keyword to set that right after the .text section.
     * (Look at the LD documentation. (Optional Section Attributes))
     * Thanks https://github.com/utzig for the hints!
     */
    .data : AT (_data_start_flash)
    {
        _data_start = .;  /* An index to the beginning of .data segment. */
        *(.data*)         /* Initialised program data */
        _data_end = .;    /* And another index to the end of .data segment. */
    } > SRAM

    /*
    * .bss contains the unitialized variables and must be set as 0x0 during runtime.
    * It should be loaded in RAM and particular care should be taken initializing them in the startup file.
    */
    .bss :
    {
        _bss_start = .;  /* This is an index to the beginning of .bss segment. */
        *(.bss*)         /* The un-initialized data should go there.  */
        _bss_end = .;    /* End index for .bss segment */
    } > SRAM

    _heap_start = .;
    _heap_end = ORIGIN(SRAM) + LENGTH(SRAM) - _stack_size;
    _stack_bottom = _heap_end;
    _stack_top = ORIGIN(SRAM) + LENGTH(SRAM);
}
