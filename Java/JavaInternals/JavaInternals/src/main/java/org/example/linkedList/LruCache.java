package org.example.linkedList;

import java.util.HashMap;
import java.util.Map;

public class LruCache {
    // Problem statement:
    // Implement the Least Recently Used (LRU) cache class LRUCache. The class should support the following operations
    //    LRUCache(int capacity) Initialize the LRU cache of size capacity.
    //    int get(int key) Return the value corresponding to the key if the key exists, otherwise return -1.
    //    void put(int key, int value) Update the value of the key if the key exists. Otherwise, add the key-value pair to the cache. If the introduction of the new pair causes the cache to exceed its capacity, remove the least recently used key.
    //
    // A key is considered used if a get or a put operation is called on it.
    // Ensure that get and put each run in O(1) average time complexity.

    // Example 1:
    // Input:
    // ["LRUCache", [2], "put", [1, 10],  "get", [1], "put", [2, 20], "put", [3, 30], "get", [2], "get", [1]]
    // Output:
    // [null, null, 10, null, null, 20, -1]

    // Explanation:
    // LRUCache lRUCache = new LRUCache(2);
    // lRUCache.put(1, 10);  // cache: {1=10}
    // lRUCache.get(1);      // return 10
    // lRUCache.put(2, 20);  // cache: {1=10, 2=20}
    // lRUCache.put(3, 30);  // cache: {2=20, 3=30}, key=1 was evicted
    // lRUCache.get(2);      // returns 20
    // lRUCache.get(1);      // return -1 (not found)

    // Constraints:
    //    1 <= capacity <= 3000
    //    0 <= key <= 10^4
    //    0 <= value <= 10^5
    //    At most 2 * 10^5 calls will be made to get and put.

    int capacity;
    DoublyLinkedList head;
    DoublyLinkedList tail;
    HashMap<Integer, DoublyLinkedList> keyValuePairs;
    int size;

    public LruCache(int capacity) {
        this.capacity = capacity;
        this.size = 0;
        this.keyValuePairs = new HashMap<>();
    }

    public int get(int key) {
        if (!this.keyValuePairs.containsKey(key)) {
            return -1;
        }

        // get the element and put it at head
        DoublyLinkedList item = this.keyValuePairs.get(key);
        // patch up the head and tail
        if (item != this.head && item != this.tail) { // item in middle, join the prev and next before removing this
            DoublyLinkedList prev = item.prev;
            DoublyLinkedList next = item.next;

            item.prev = null;
            item.next = null;

            prev.next = next;
            next.prev = prev;

            // put the item at the head
            DoublyLinkedList nextOne = this.head;
            this.head = item;
            item.next = nextOne;
            nextOne.prev = item;
        } else if (item == this.tail) { // last item, just cut the reference from the left
            DoublyLinkedList prev = item.prev;
            prev.next = null;
            item.prev = null;
            this.tail = prev;       // assign the new tail

            // put the item at the head
            DoublyLinkedList nextOne = this.head;
            this.head = item;
            item.next = nextOne;
            nextOne.prev = item;
        }
//        else { // must be the first item, can leave as is
//
//        }

        // now can return the value
        return item.val;
    }

    public void put(int key, int value) {
        DoublyLinkedList thisOne;

        if (this.keyValuePairs.containsKey(key)) {   // if the value already exists, we replace it
            thisOne = this.keyValuePairs.get(key);
            if (thisOne.next != null && thisOne.prev != null) {
                DoublyLinkedList prev = thisOne.prev;
                DoublyLinkedList next = thisOne.next;

                prev.next = next;
                next.prev = prev;

                thisOne.val = value;
            }
        } else {
            // if new create and push
            thisOne = new DoublyLinkedList(value, key);
            this.keyValuePairs.put(key, thisOne);

            this.size += 1;
        }

        // set this value at the head
        DoublyLinkedList oldHead = this.head;
        this.head = thisOne;
        thisOne.next = oldHead;
        if (oldHead != null) oldHead.prev = thisOne;

        if (oldHead != null && oldHead.next == null) {
            this.tail = oldHead;
        }

        // remove if the size exceeds capacity
        if (this.size > this.capacity) {
            DoublyLinkedList currentTail = this.tail;
            this.keyValuePairs.remove(currentTail.key);

            DoublyLinkedList prev = currentTail.prev;
            prev.next = null;
            currentTail.prev = null;
            this.tail = prev;
            this.size -= 1;
        }
    }

    public String doublyListAsString() {
        StringBuilder sb = new StringBuilder();
        DoublyLinkedList head = this.head;
        sb.append("(");
        while (head != null) {
            sb.append("\n(id=" + head.id + ", val=" + head.val + ", perv: " + (head.prev != null ? head.prev.id : "null") + ", next: " + (head.next != null ? head.next.id : "null") + "),");
            head = head.next;
        }
        sb.append("\n)");
        return sb.toString();
    }

    @Override
    public String toString() {
        return "(cap=" + this.capacity + ", size=" + this.size + ", head=(id=" +
                (this.head != null ? this.head.id : "null") + ", val=" +
                (this.head != null ? this.head.val : "null") + "), tail=(id=" +
                (this.tail != null ? this.tail.id : "null") + ", val=" +
                (this.tail != null ? this.tail.val : "null") + "))";
    }

    public static void main(String[] args) {
        System.out.println("here goes everything...");

//        DoublyLinkedList l1 = new DoublyLinkedList(1);
//        DoublyLinkedList l2 = new DoublyLinkedList(2);
//        DoublyLinkedList l3 = new DoublyLinkedList(3);
//        l1.next = l2;
//        l2.prev = l1;
//        l2.next = l3;
//        l3.prev = l2;
//
//        System.out.println("l1: " + l1);
//        System.out.println("l2: " + l2);
//        System.out.println("l3: " + l3);

        LruCache cache = new LruCache(2);
        System.out.println("lru: " + cache);
        cache.put(1, 10);
        System.out.println("get(1): " + cache.get(1));
        cache.put(2, 20);
        cache.put(3, 30);
        System.out.println("get(2): " + cache.get(2));
        System.out.println("get(1): " + cache.get(1));
        System.out.println("the whole dll: " + cache.doublyListAsString());
    }

    public static class DoublyLinkedList {
        int id;
        int val;
        int key;
        DoublyLinkedList prev;
        DoublyLinkedList next;

        public DoublyLinkedList(int val, int key) {
            this.val = val;
            this.id = LruCache.generateId();
            this.key = key;
        }

        @Override
        public String toString() {
            return "(id=" + this.id + ", val=" + this.val + ", prevId=" +
                    (this.prev != null ? this.prev.id : "null") + ", nextId=" +
                    (this.next != null ? this.next.id : "null") + ")";
        }
    }

    static int id = 0;
    static int generateId() {
        int newId = id + 1;
        LruCache.id = newId;
        return newId;
    }
}
