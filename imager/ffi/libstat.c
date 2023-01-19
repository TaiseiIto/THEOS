#include <sys/stat.h>
#include <time.h>
#include <unistd.h>

typedef struct _TimeSpec {
	unsigned long long int tv_sec;
	unsigned long int tv_nsec;
} TimeSpec;

TimeSpec get_accessed_time(char const *path) {
	TimeSpec time_spec;
	struct stat st;
	stat(path, &st);
	time_spec.tv_sec = st.st_atim.tv_sec;
	time_spec.tv_nsec = st.st_atim.tv_nsec;
	return time_spec;
}

TimeSpec get_changed_time(char const *path) {
	TimeSpec time_spec;
	struct stat st;
	stat(path, &st);
	time_spec.tv_sec = st.st_ctim.tv_sec;
	time_spec.tv_nsec = st.st_ctim.tv_nsec;
	return time_spec;
}

TimeSpec get_modified_time(char const *path) {
	TimeSpec time_spec;
	struct stat st;
	stat(path, &st);
	time_spec.tv_sec = st.st_mtim.tv_sec;
	time_spec.tv_nsec = st.st_mtim.tv_nsec;
	return time_spec;
}

TimeSpec get_current_time() {
	struct timespec time_spec_src;
	TimeSpec time_spec_dst;
	clock_gettime(CLOCK_REALTIME, &time_spec_src);
	time_spec_dst.tv_sec = time_spec_src.tv_sec;
	time_spec_dst.tv_nsec = time_spec_src.tv_nsec;
	return time_spec_dst;
}

