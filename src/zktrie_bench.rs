use std::prelude::v1::*;

use std::fs::File;
use std::io::Read;

use crate::byte32_test::TestHash;
use crate::{to_secure_key, Byte32, Hash, MemDB, ZkTrie};

#[bench]
fn bench_trie_add(b: &mut test::Bencher) {
    let mut db = MemDB::new();
    let db = &mut db;
    let mut trie = <ZkTrie<TestHash>>::new(248, Hash::default());
    let mut ds = Datasource::new(102400);

    b.iter(|| {
        let values = ds.get_vec(5);
        let key = ds.get();
        trie.update(db, key.bytes(), 1, values).unwrap();
    });
}

#[bench]
fn bench_trie_get(b: &mut test::Bencher) {
    let mut db = MemDB::new();
    let db = &mut db;
    let mut ds = Datasource::new(102400);

    let mut trie = <ZkTrie<TestHash>>::new(248, Hash::default());

    let mut keys = vec![];
    for _ in 0..10240 {
        let key = ds.get();
        let values = ds.get_vec(5);
        trie.update(db, key.bytes(), 1, values).unwrap();
        keys.push(key);
    }
    let mut key_idx = 0;
    let mut get_key = || {
        key_idx += 1;
        keys[key_idx % keys.len()]
    };

    b.iter(|| {
        trie.get_data(db, get_key().bytes()).unwrap();
    });
}

#[bench]
fn bench_trie_deletion(b: &mut test::Bencher) {
    let mut db = MemDB::new();
    let db = &mut db;
    let mut ds = Datasource::new(102400);

    let mut trie = <ZkTrie<TestHash>>::new(248, Hash::default());

    let mut keys = vec![];
    for _ in 0..10240 {
        let key = ds.get();
        let values = ds.get_vec(5);
        trie.update(db, key.bytes(), 1, values).unwrap();
        keys.push(key);
    }
    let mut key_idx = 0;
    let mut get_key = || {
        key_idx += 1;
        keys[key_idx % keys.len()]
    };

    b.iter(|| {
        trie.delete(db, get_key().bytes()).unwrap();
    });
}

pub struct Datasource {
    db: Vec<Byte32>,
    idx: usize,
}

impl Datasource {
    pub fn new(n: usize) -> Datasource {
        let mut fd = File::open("/dev/random").unwrap();
        let mut tmp = [0_u8; 32];
        let mut db = Vec::with_capacity(n);
        for _ in 0..n {
            fd.read(&mut tmp).unwrap();
            let hash: Hash = to_secure_key::<TestHash>(&tmp).unwrap().into();
            let item: Byte32 = hash.bytes().into();
            db.push(item);
        }
        Datasource { db, idx: 0 }
    }

    pub fn get(&mut self) -> Byte32 {
        self.idx += 1;
        self.db[self.idx % self.db.len()]
    }

    pub fn get_vec(&mut self, n: usize) -> Vec<Byte32> {
        let mut out = Vec::with_capacity(n);
        for _ in 0..n {
            out.push(self.get());
        }
        out
    }
}

/* Go Version

func BenchmarkTrieAdd(b *testing.B) {
	db := NewZkTrieMemoryDb()
	root := zkt.Byte32{}
	zkTrie, err := NewZkTrie(root, db)
	if err != nil {
		b.Fatalf("create zktrie fail: %v", err)
	}

	ds := NewDatasource(102400)

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		key := ds.Get()
		values := ds.GetSlice(5)
		err := zkTrie.TryUpdate(key[:], 1, values)
		if err != nil {
			b.Fatal(err)
		}
	}
}

func BenchmarkTrieGet(b *testing.B) {
	db := NewZkTrieMemoryDb()
	root := zkt.Byte32{}
	zkTrie, err := NewZkTrie(root, db)
	if err != nil {
		b.Fatalf("create zktrie fail: %v", err)
	}

	ds := NewDatasource(102400)

	var keys []zkt.Byte32
	for i := 0; i < 10240; i++ {
		key := ds.Get()
		values := ds.GetSlice(5)
		if err := zkTrie.TryUpdate(key[:], 1, values); err != nil {
			b.Fatal(err)
		}
		keys = append(keys, key)
	}

	keyIdx := 0
	getKey := func() zkt.Byte32 {
		keyIdx += 1
		return keys[keyIdx%len(keys)]
	}

	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		key := getKey()
		if _, err := zkTrie.TryGet(key[:]); err != nil {
			b.Fatal(err)
		}
	}
}

func BenchmarkTrieDeletion(b *testing.B) {
	db := NewZkTrieMemoryDb()
	root := zkt.Byte32{}
	zkTrie, err := NewZkTrie(root, db)
	if err != nil {
		b.Fatalf("create zktrie fail: %v", err)
	}

	ds := NewDatasource(102400)

	var keys []zkt.Byte32
	for i := 0; i < 10240; i++ {
		key := ds.Get()
		values := ds.GetSlice(5)
		if err := zkTrie.TryUpdate(key[:], 1, values); err != nil {
			b.Fatal(err)
		}
		keys = append(keys, key)
	}

	keyIdx := 0
	getKey := func() zkt.Byte32 {
		keyIdx += 1
		return keys[keyIdx%len(keys)]
	}

	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		key := getKey()
		if err := zkTrie.TryDelete(key[:]); err != nil {
			b.Fatal(err)
		}
	}
}

type Datasource struct {
	db  []zkt.Byte32
	idx int
}

func NewDatasource(n int) Datasource {
	db := make([]zkt.Byte32, n)
	for i := 0; i < n; i++ {
		rand.Read(db[i][:])
	}
	return Datasource{
		db:  db,
		idx: 0,
	}
}

func (d *Datasource) Get() zkt.Byte32 {
	d.idx += 1
	return d.db[d.idx%len(d.db)]
}

func (d *Datasource) GetSlice(n int) []zkt.Byte32 {
	out := make([]zkt.Byte32, n)
	for i := 0; i < n; i++ {
		out[i] = d.Get()
	}
	return out
}

 */