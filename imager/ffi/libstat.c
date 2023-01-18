#include <sys/stat.h>
#include <time.h>
#include <unistd.h>

typedef struct _TimeSpec {
	unsigned long long int tv_sec;
	unsigned long int tv_nsec;
} __attribute__((packed)) TimeSpec;

time_t get_accessed_time_sec(char const *path) {
	struct stat st;
	stat(path, &st);
	return st.st_atim.tv_sec;
}

long get_accessed_time_nsec(char const *path) {
	struct stat st;
	stat(path, &st);
	return st.st_atim.tv_nsec;
}

time_t get_changed_time_sec(char const *path) {
	struct stat st;
	stat(path, &st);
	return st.st_ctim.tv_sec;
}

long get_changed_time_nsec(char const *path) {
	struct stat st;
	stat(path, &st);
	return st.st_ctim.tv_nsec;
}

time_t get_modified_time_sec(char const *path) {
	struct stat st;
	stat(path, &st);
	return st.st_mtim.tv_sec;
}

long get_modified_time_nsec(char const *path) {
	struct stat st;
	stat(path, &st);
	return st.st_mtim.tv_nsec;
}

TimeSpec get_current_time() {
	struct timespec time_spec_src;
	TimeSpec time_spec_dst;
	clock_gettime(CLOCK_REALTIME, &time_spec_src);
	time_spec_dst.tv_sec = time_spec_src.tv_sec;
	time_spec_dst.tv_nsec = time_spec_src.tv_nsec;
	return time_spec_dst;
}

