#include <filesystem>
#include <iostream>
#include "directory.hpp"

Directory::Directory(std::string path)
{
	for(std::filesystem::directory_iterator i = std::filesystem::begin(std::filesystem::directory_iterator(path)); i != std::filesystem::end(std::filesystem::directory_iterator(path)); i++)std::cout << i->path() << std::endl;
}

