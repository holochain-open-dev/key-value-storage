import { Orchestrator, Config, InstallAgentsHapps } from "@holochain/tryorama";
import path from "path";

const conductorConfig = Config.gen();

// Construct proper paths for your DNAs
const storageDna = path.join(
  __dirname,
  "../../storage.dna.workdir/key-value-storage.dna"
);
console.log("storageDna", storageDna);

// create an InstallAgentsHapps array with your DNAs to tell tryorama what
// to install into the conductor.
const installation: InstallAgentsHapps = [
  // agent 0
  [
    // happ 0
    [storageDna],
  ],
];

const sleep = (ms: number) =>
  new Promise((resolve) => setTimeout(() => resolve(true), ms));

const orchestrator = new Orchestrator();

orchestrator.registerScenario("create an entry and get it", async (s, t) => {
  const [alice] = await s.players([conductorConfig]);

  // install your happs into the coductors and destructuring the returned happ data using the same
  // array structure as you created in your installation array.
  const [[alice_common]] = await alice.installAgentsHapps(installation);

  // 🌈
  // Create store1
  let store1 = await alice_common.cells[0].call("storage", "create_store", {
    store: "store1",
  });
  t.ok(store1, `create_store 1 : ${JSON.stringify(store1, undefined, 2)}`);
  await sleep(500);

  // 🌈
  // Get store1
  let getStore1 = await alice_common.cells[0].call("storage", "get_store", {
    store: "store1",
  });
  t.deepEqual(
    store1,
    getStore1,
    `get_store 1 : ${JSON.stringify(getStore1, undefined, 2)}`
  );

  // 🌈
  // Create store1 - ALREADY EXISTS
  try {
    await alice_common.cells[0].call("storage", "create_store", {
      store: "store1",
    });
    t.fail("Function should throw exeption");
  } catch (e) {
    t.ok(e, "Exeption thrown as expected, store already exists");
  }

  // 🌈
  // Create store1 - NO NAME
  try {
    await alice_common.cells[0].call("storage", "create_store", {
      store: "",
    });
    t.fail("Function should throw exeption");
  } catch (e) {
    t.ok(e, "Exeption thrown as expected, empty name not allowed");
  }

  // 🌈
  // Create store1 - TOO LONG NAME
  try {
    await alice_common.cells[0].call("storage", "create_store", {
      store: "12345678901234567890123456789012345678901234567890X",
    });
    t.fail("Function should throw exeption");
  } catch (e) {
    t.ok(e, "Exeption thrown as expected, max 50 characters in name");
  }

  // 🌈
  // Create store2
  let store2 = await alice_common.cells[0].call("storage", "create_store", {
    store: "store2",
  });
  t.ok(store2, `create_store 2 : ${JSON.stringify(store2, undefined, 2)}`);
  await sleep(500);

  // 🌈
  // Get store - NOT FOUND
  try {
    await alice_common.cells[0].call("storage", "get_store", {
      store: "not_found",
    });
    t.fail("Function should throw exeption");
  } catch (e) {
    t.ok(e, "Exeption thrown as expected, store already exists");
  }

  // 🌈
  // Set item - STORE DOESN'T EXIST
  try {
    await alice_common.cells[0].call("storage", "set_item", {
      store: "doesnt_exist",
      key: "key1",
      value: "value1",
    });
    t.fail("Function should throw exeption");
  } catch (e) {
    t.ok(e, "Exeption thrown as expected, store doesn't exist");
  }

  // 🌈
  // Set item - KEY TOO SHORT
  try {
    await alice_common.cells[0].call("storage", "set_item", {
      store: "store1",
      key: "",
      value: "value1",
    });
    t.fail("Function should throw exeption");
  } catch (e) {
    t.ok(e, "Exeption thrown as expected, key too short");
  }

  // 🌈
  // Set item - KEY TOO LONG
  try {
    await alice_common.cells[0].call("storage", "set_item", {
      store: "store1",
      key: "12345678901234567890123456789012345678901234567890X",
      value: "value1",
    });
    t.fail("Function should throw exeption");
  } catch (e) {
    t.ok(e, "Exeption thrown as expected, key too long");
  }

  // 🌈
  // Set item - VALUE TOO SHORT
  try {
    await alice_common.cells[0].call("storage", "set_item", {
      store: "store1",
      key: "key1",
      value: "",
    });
    t.fail("Function should throw exeption");
  } catch (e) {
    t.ok(e, "Exeption thrown as expected, value too short");
  }

  // 🌈
  // Set item - VALUE TOO LONG
  try {
    await alice_common.cells[0].call("storage", "set_item", {
      store: "store1",
      key: "key1",
      value:
        "123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890",
    });
    t.fail("Function should throw exeption");
  } catch (e) {
    t.ok(e, "Exeption thrown as expected, value too long");
  }

  // 🌈
  // Set item 1
  let item1 = await alice_common.cells[0].call("storage", "set_item", {
    store: "store1",
    key: "key1",
    value: "value1",
  });
  t.ok(item1, `set_item 1 : ${JSON.stringify(item1, undefined, 2)}`);
  await sleep(500);

  // 🌈
  // Get item 1
  let getItem1 = await alice_common.cells[0].call("storage", "get_item", {
    store: "store1",
    key: "key1",
  });
  t.deepEqual(
    item1,
    getItem1,
    `get_item 1 : ${JSON.stringify(getItem1, undefined, 2)}`
  );

  // 🌈
  // Set item 1 AGAIN
  item1 = await alice_common.cells[0].call("storage", "set_item", {
    store: "store1",
    key: "key1",
    value: "value1",
  });
  t.ok(item1, `set_item 1 AGAIN : ${JSON.stringify(item1, undefined, 2)}`);
  await sleep(500);

  // 🌈
  // Get item 1 AGAIN
  getItem1 = await alice_common.cells[0].call("storage", "get_item", {
    store: "store1",
    key: "key1",
  });
  t.deepEqual(
    item1,
    getItem1,
    `get_item 1 AGAIN : ${JSON.stringify(getItem1, undefined, 2)}`
  );

  // 🌈
  // Set item 2
  let item2 = await alice_common.cells[0].call("storage", "set_item", {
    store: "store1",
    key: "key2",
    value: "value2",
  });
  t.ok(item2, `set_item 2 : ${JSON.stringify(item2, undefined, 2)}`);
  await sleep(500);

  // 🌈
  // Set item 3
  let item3 = await alice_common.cells[0].call("storage", "set_item", {
    store: "store1",
    key: "key3",
    value: "value3",
  });
  t.ok(item3, `set_item 3 : ${JSON.stringify(item2, undefined, 2)}`);
  await sleep(500);

  // 🌈
  // Remove item 3
  let removeItem3 = await alice_common.cells[0].call("storage", "remove_item", {
    store: "store1",
    key: "key3",
  });
  t.false(removeItem3, `remove_item 3`);
  await sleep(500);

  // 🌈
  // List keys
  let keys = await alice_common.cells[0].call("storage", "keys", {
    store: "store1",
  });
  t.deepEqual(
    keys,
    { keys: [{ key: "key1" }, { key: "key2" }] },
    `keys : ${JSON.stringify(keys, undefined, 2)}`
  );

  // 🌈
  // Get store length
  let length = await alice_common.cells[0].call("storage", "length", {
    store: "store1",
  });
  t.equal(length.length, 2, `length : ${JSON.stringify(length, undefined, 2)}`);

  // 🌈
  // Get item 1, using index 0
  let key = await alice_common.cells[0].call("storage", "key", {
    store: "store1",
    index: 0,
  });
  t.deepEqual(
    key,
    { key: "key1" },
    `get_item using index 0 : ${JSON.stringify(key, undefined, 2)}`
  );

  // 🌈
  // Get item key - index to big
  try {
    getItem1 = await alice_common.cells[0].call("storage", "key", {
      store: "store1",
      index: 10,
    });
    t.fail("Function should throw exeption");
  } catch (e) {
    t.ok(e, "Exeption thrown as expected, index too big");
  }

  // 🌈
  // Clear store
  let clear = await alice_common.cells[0].call("storage", "clear", {
    store: "store1",
  });
  t.false(clear, `clear`);
  await sleep(500);

  // 🌈
  // Get store length
  length = await alice_common.cells[0].call("storage", "length", {
    store: "store1",
  });
  t.equal(length.length, 0, `length : Should be 0`);

  // 🌈
  // List keys on empty store
  keys = await alice_common.cells[0].call("storage", "keys", {
    store: "store1",
  });
  t.equal(keys.keys.length, 0, `keys.length : Should be 0`);

  // // 🌈
  // // Set 100 items
  // console.log("Creating 100 items.");
  // for (let i = 0; i < 100; i++) {
  //   await alice_common.cells[0].call("storage", "set_item", {
  //     store: "store1",
  //     key: "key" + i,
  //     value: "value",
  //   });
  // }
  // await sleep(500);

  // // 🌈
  // // Get store length
  // length = await alice_common.cells[0].call("storage", "length", {
  //   store: "store1",
  // });
  // t.equal(length.length, 100, `length : Should be 100`);

  // // 🌈
  // // Clear store
  // clear = await alice_common.cells[0].call("storage", "clear", {
  //   store: "store1",
  // });
  // t.false(clear, `clear`);
  // await sleep(500);

  // // 🌈
  // // Get store length
  // length = await alice_common.cells[0].call("storage", "length", {
  //   store: "store1",
  // });
  // t.equal(length.length, 0, `length : Should be 0`);
});

orchestrator.run();
