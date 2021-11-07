# Disk layout
1 sector consists of 0x200 bytes.
And 1 cluster consists of 0x8 sectors.
So, 1 cluster is 4KiB.
In addition, the disk image consists of 0x100 clusters.
Therefore, the disk size is 1 MiB.

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
* cluster 0x04 : First FAT
* cluster 0x05 : Second FAT
* cluster 0x06~0xff : Data Area

The sub directory `reserved_area` generates the reserved area of the disk, from cluster 0x00 to cluster 0x03.

