# Reduce UDF

Reduce is one of the most commonly used abstractions in a stream processing pipeline to define 
aggregation functions on a stream of data. It is the reduce feature that helps us solve problems like 
"performs a summary operation(such as counting the number of occurrence of a key, yielding user login 
frequencies), etc."Since the input an unbounded stream (with infinite entries), we need an additional
parameter to convert the unbounded problem to a bounded problem and provide results on that. That
bounding condition is "time", eg, "number of users logged in per minute". So while processing an 
unbounded stream of data, we need a way to group elements into finite chunks using time. To build these
chunks the reduce function is applied to the set of records produced using the concept of [windowing](./windowing/windowing.md).

Unlike in _map_ vertex where only an element is given to user-defined function, in _reduce_ since
there is a group of elements, an iterator is passed to the reduce function. The following is a generic
outlook of a reduce function. I have written the pseudo-code using the accumulator to show that very
powerful functions can be applied using this reduce semantics.

```python
# reduceFn func is a generic reduce function that processes a set of elements
def reduceFn(key: str, datums: Iterator[Datum], md: Metadata) -> Messages:
    # initialize_accumalor could be any function that starts of with an empty
    # state. eg, accumulator = 0
    accumulator = initialize_accumalor()
    # we are iterating on the input set of elements
    for d in datums:
        # accumulator.add_input() can be any function. 
        # e.g., it could be as simple as accumulator += 1
        accumulator.add_input(d)
    # once we are done with iterating on the elements, we return the result
    # acumulator.result() can be str.encode(accumulator)
    return Messages(Message.to_vtx(key, acumulator.result()))
```

The structure for defining a reduce vertex is as follows.
```yaml
- name: my-reduce-udf
  udf:
    container:
      image: my-reduce-udf:latest
    groupBy:
      window:
        ...
      keyed: ...
      storage:
        ...
```

The reduce spec adds a new section called `groupBy` and this how we differentiate a _map_ vertex
from _reduce_ vertex. There are two important fields, the [_window_](./windowing/windowing.md)
and [_keyed_](./windowing/windowing.md#non-keyed-vs-keyed-windows). These two fields play an
important role in grouping the data together and pass it to the user-defined reduce code.

The reduce supports a parallelism value while defining the edge. This is because auto-scaling is 
not supported in reduce vertex. If `parallelism` is not defined default of one will be used.

```yaml
- from: edge1
  to: my-reduce-reduce
  parallelism: integer
```

It is wrong to give a `parallelism` > `1` if it is a _non-keyed_ vertex (`keyed: false`).

There are a couple of [examples](examples.md) that demonstrates Fixed windows, Sliding windows,
chaining of windows, keyed streams, etc.

## Storage

Reduce unlike map requires persistence. To support persistence user has to define the 
`storage` configuration. We replay the data stored in this storage on pod startup if there has
been a restart of the reduce pod caused due to pod migrations, etc.

```yaml
vertices:
  - name: my-udf
    udf:
      groupBy:
        storage:
           ....
```

### Persistent Volume Claim (PVC)

`persistentVolumeClaim` supports the following fields, `volumeSize`, `storageClassName`, and`accessMode`.
As name suggests,`volumeSize` specifies the size of the volume. `accessMode` can be of many types, but for 
reduce usecase we need only `ReadWriteOnce`. `storageClassName` can also be provided, more info on storage class
can be found [here](https://kubernetes.io/docs/concepts/storage/persistent-volumes#class-1). The default
value of `storageClassName` is `default` which is default StorageClass may be deployed to a Kubernetes 
cluster by addon manager during installation.

#### Example
```yaml
vertices:
  - name: my-udf
    udf:
      groupBy:
        storage:
          persistentVolumeClaim:
            volumeSize: 10Gi
            accessMode: ReadWriteOnce
```

### EmptyDir 

We also support `emptyDir` for quick experimentation. We do not recommend this in production
setup. If we use `emptyDir`, we will end up in data loss if there are pod migrations. `emptyDir` 
also takes an optional `sizeLimit`. `medium` field controls where emptyDir volumes are stored.
By default emptyDir volumes are stored on whatever medium that backs the node such as disk, SSD, 
or network storage, depending on your environment. If you set the `medium` field to `"Memory"`, 
Kubernetes mounts a tmpfs (RAM-backed filesystem) for you instead.

#### Example

```yaml
vertices:
  - name: my-udf
    udf:
      groupBy:
        storage:
          emptyDir: {}
```