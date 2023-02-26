#include <Uefi.h>
#include <Library/UefiLib.h>

EFI_STATUS EFIAPI Main(EFI_HANDLE image_handle, EFI_SYSTEM_TABLE *system_table) {
	Print(L"Hello, World!\n");
	while(1);
	return EFI_SUCCESS;
}

