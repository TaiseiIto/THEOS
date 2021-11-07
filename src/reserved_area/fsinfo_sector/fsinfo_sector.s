	.set	fsinfo_sign_1,	0x41615252
	.set	fsinfo_sign_2,	0x61417272
	.set	free_clusters,	0xffffffff
	.set	last_cluster,	0xffffffff
	.set	fsinfo_sign_3,	0xaa550000

	.data
	.long	fsinfo_sign_1
	.fill	0x1e0, 0x01, 0x00	# Locate 0x1e0 bytes of 0x00
	.long	fsinfo_sign_2
	.long	free_clusters
	.long	last_cluster
	.fill	0x0c, 0x01, 0x00	# Locate 0x0c bytes of 0x00
	.long	fsinfo_sign_3

