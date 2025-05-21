Consider adding creep options to handle creeps are a more granular level based on room state.
An example from our previous code base was having an option in the creep's memory that allowed them to
get energy from links, storage, containers, etc. This let us turn off and on certain features of creeps depending on the state of affairs.
  This would come very in handy for boostraping carriers. Rather than having to create a whole new role that can mine for energy,
we could simply flip on the "harvestSource" option on the carriers that are spawned and they would be able to quickly kickstart the room

Might want to consider a system to get objects of varying types from the room api, like sources, containers, mining containers, storage, etc. Having a consistent interface here would clean up the code of a lot of searches. We might also want to consider caching these values in the heap

At some point when the code base has somewhat stablized in it's patterns, need to go through and give some attention to all the unwraps and unhandled results

Want to consider pre-loading jobs instead of creating them on demand. The reasoning here is so we can not double up on jobs such as filling structures