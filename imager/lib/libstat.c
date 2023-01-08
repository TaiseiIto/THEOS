#include <sys/stat.h>
#include <time.h>
#include <unistd.h>

time_t get_access_time_sec(char const *path) {
	struct stat st;
	stat(path, &st);
	return st.st_atim.tv_sec;
}

long get_access_time_nsec(char const *path) {
	struct stat st;
	stat(path, &st);
	return st.st_atim.tv_nsec;
}

time_t get_change_time_sec(char const *path) {
	struct stat st;
	stat(path, &st);
	return st.st_ctim.tv_sec;
}

long get_change_time_nsec(char const *path) {
	struct stat st;
	stat(path, &st);
	return st.st_ctim.tv_nsec;
}

time_t get_modification_time_sec(char const *path) {
	struct stat st;
	stat(path, &st);
	return st.st_mtim.tv_sec;
}

long get_modification_time_nsec(char const *path) {
	struct stat st;
	stat(path, &st);
	return st.st_mtim.tv_nsec;
}

