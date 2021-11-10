#ifndef _DIRECTORY_HPP_
#define _DIRECTORY_HPP_

#include <string>

class Directory
{
	std::string input_path; // directory path on the source repository
	std::string output_path; // directory path on the disk image
public:
	Directory(std::string path);
};

#endif

