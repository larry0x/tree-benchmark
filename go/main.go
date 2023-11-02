package main

import (
	"encoding/json"
	"fmt"
	"os"
	"time"

	"cosmossdk.io/log"
	dbm "github.com/cosmos/cosmos-db"
	"github.com/cosmos/iavl"
)

const (
	dbPath = "../testdata/db/iavl"

	// 50mb, the default used by for cosmos-sdk
	cacheSize = 781250
)

type Op struct {
	Insert string `json:"insert"`
}

type Batch = map[string]Op

func main() {
	batchesBz, err := os.ReadFile("../testdata/batches.json")
	if err != nil {
		panic(nil)
	}

	batches := []Batch{}
	if err = json.Unmarshal(batchesBz, &batches); err != nil {
		panic(err)
	}

	keysBz, err := os.ReadFile("../testdata/keys.json")
	if err != nil {
		panic(nil)
	}

	keys := []string{}
	if err = json.Unmarshal(keysBz, &keys); err != nil {
		panic(err)
	}

	// delete the DB if already exists. we start from a blank state
	if _, err = os.Stat(dbPath); os.IsNotExist(err) {
	} else {
		if err = os.RemoveAll(dbPath); err != nil {
			panic(err)
		}
	}

	db, err := dbm.NewGoLevelDB("iavl-benchmark-db", dbPath, nil)
	if err != nil {
		panic(err)
	}

	tree := iavl.NewMutableTree(db, cacheSize, true, log.NewNopLogger())

	// bench writes
	fmt.Println("[1] writing to disk")
	startTime := time.Now()
	numWrites := 0
	for _, batch := range batches {
		// write the data to the tree. at this point the diffs are in memory only
		for key, op := range batch {
			numWrites += 1
			if _, err = tree.Set([]byte(key), []byte(op.Insert)); err != nil {
				panic(err)
			}
		}

		// persist the writes to disk
		if _, _, err = tree.SaveVersion(); err != nil {
			panic(err)
		}
	}
	elapsedTime := time.Since(startTime)
	fmt.Println("elapsed time: ", elapsedTime)
	fmt.Println("num writes:   ", numWrites)
	fmt.Println("writes/second:", float64(numWrites)/elapsedTime.Seconds())

	// bench reads
	fmt.Println("[2] reading from disk")
	startTime = time.Now()
	for _, key := range keys {
		if _, err = tree.Get([]byte(key)); err != nil {
			panic(err)
		}
	}
	elapsedTime = time.Since(startTime)
	fmt.Println("elapsed time:", elapsedTime)
	fmt.Println("num reads:   ", len(keys))
	fmt.Println("reads/second:", float64(len(keys))/elapsedTime.Seconds())
}
