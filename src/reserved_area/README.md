# FAT32 reserved area layout

1 sector consists of 0x200 bytes.
And 1 cluster consists of 0x8 sectors.
So, 1 cluster is 4KiB.

* cluster 0x00 : Boot cluster
    * sector 0x00 : Boot sector
    * sector 0x01 : FSInfo sector
    * sector 0x02 : Empty
    * sector 0x03 : Empty
    * sector 0x04 : Empty
    * sector 0x05 : Empty
    * sector 0x06 : Backup boot sector
    * sector 0x07 : Backup FSInfo sector
* cluster 0x01 : Empty
* cluster 0x02 : Empty
* cluster 0x03 : Empty

There are 2 important sectors, boot sector and FSInfo sector in sector 0x00,0x06 and sector 0x01,0x07 of cluster 0x00, respectively.
As you see, each sector is deployed 2 different places for redundancy.
And the sub directories `boot_sector` and `fsinfo_sector` generate raw image binaries of each sector.
Then, `builder` generated from the sub directory `builder` combines these sectors and generates a raw image of the reserved area.

