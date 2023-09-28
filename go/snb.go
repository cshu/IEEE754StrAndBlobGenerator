package main

import (
	"fmt"
	rs "github.com/cshu/golangrs"
	"os"
)

func main() {
	const filenm = `/tmp/tmp_ieee754_str_n_blob`
	switch os.Args[1] {
	case `generate`:
		rs.IEEE754GenerateRandomPairStrAndBlob(42, filenm, 123, true)
		fmt.Println(`Generated`)
	case `verify`:
		rs.IEEE754VerifyPairStrAndBlob(filenm, true)
		fmt.Println(`Verified`)
	default:
		fmt.Println(`Do nothing`)
	}
}
