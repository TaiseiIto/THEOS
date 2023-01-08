#include <sys/stat.h>
#include <time.h>
#include <unistd.h>

time_t get_access_time(char const *path) {
	struct stat st;
	stat(path);
	return st.st_atime;
}

time_t get_change_itime(char const *path) {
	struct stat st;
	stat(path);
	return st.st_ctime;
}

time_t get_modification_time(char const *path) {
	struct stat st;
	stat(path);
	return st.st_mtime;
}

