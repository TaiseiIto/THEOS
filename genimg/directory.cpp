#include <filesystem>
#include <iostream>
#include "directory.hpp"

Directory::Directory(std::string path): input_path(path), output_path("/")
{
	std::filesystem::directory_iterator my_iterator = std::filesystem::directory_iterator(path);
	for(std::filesystem::directory_iterator i = std::filesystem::begin(my_iterator); i != std::filesystem::end(my_iterator); i++)std::cout << i->path() << std::endl;
}

