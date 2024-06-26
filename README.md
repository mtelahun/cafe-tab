# CafeTab
![Rust](https://github.com/mtelahun/cafe-tab/actions/workflows/rust.yml/badge.svg)
[![codecov](https://codecov.io/gh/mtelahun/cafe-tab/branch/main/graph/badge.svg?token=A1P9I5E2LU)](https://codecov.io/gh/mtelahun/cafe-tab)
[![License](https://img.shields.io/badge/License-BSD_2--Clause-orange.svg)](https://opensource.org/licenses/BSD-2-Clause)

From [Getting Started with CQRS](https://cqrs.nu/)

## The Domain

For this tutorial, we'll work in the cafe domain. Our focus will be on the concept of a tab, which tracks the visit of an individual or group to the cafe. When people arrive to the cafe and take a table, a tab is opened. They may then order drinks and food. Drinks are served immediately by the table staff, however food must be cooked by a chef. Once the chef has prepared the food, it can then be served.

During their time at the restaurant, visitors may order extra food or drinks. If they realize they ordered the wrong thing, they may amend the order - but not after the food and drink has been served to and accepted by them.

Finally, the visitors close the tab by paying what is owed, possibly with a tip for the serving staff. Upon closing a tab, it must be paid for in full. A tab with unserved items cannot be closed unless the items are either marked as served or cancelled first.
Events

In the scenario described above, various verbs and nouns were picked out. When working in a database-centric way, it is common to listen carefully for nouns, mapping them to tables and relating them. The verbs are thus often a secondary concern. Designing in terms of commands and events puts the focus on verbs instead, with the nouns being considered a little later. This is good, since things that make a domain interesting tend to be captured by the verbs rather than the nouns. Every business has customers (hopefully!) - it's what they do for them that matters.

Looking through the scenario, focusing on the language we find within it, we look for things that happen that lead to some kind of new information in the domain. We map these happenings to a set of events. Since events are about things that have taken place, they are named in the past tense.

Here are a set of events we may come up with from the cafe tab scenario.

    TabOpened
    DrinksOrdered
    FoodOrdered
    DrinksCancelled
    FoodCancelled
    DrinksServed
    FoodPrepared
    FoodServed
    TabClosed

Note that the events are very domain-focused. The difference between ordering drinks and ordering food matters, so we capture these into different events. Also note the verbs are from the domain, not generic terms like "Created", "Updated", or "Deleted". While having such events isn't automatically wrong, if they form the majority of what you have then it's time to look deeper at the domain, and escape CRUD-think. Or maybe what you're doing just isn't complex enough to warrant applying DDD, in which case do something cheaper.

## Commands

Commands are things that indicate requests to our domain. While an event states that something certainly happened, a command may be accepted or rejected. An accepted command leads to zero or more events being emitted to incorporate new facts into the system. A rejected command leads to some kind of exception.

Commands are also identified by looking for verbs. However, they are focused around what the user considers as an operation. For example, while it is important that food and drink are handled differently in the domain, table staff will most certainly not want to enter drinks into the system and order them, then separately enter food into the system, with no way to get an overview of the order being placed! It's likely that each person at a table will specify their food and drink together, and somebody will probably change their mind after learning what their friend orders. Therefore, there will be a single command for placing an order.

Here are the initial commands we arrive at for this domain:

    OpenTab
    PlaceOrder
    AmendOrder
    MarkDrinksServed
    MarkFoodPrepared
    MarkFoodServed
    CloseTab

Notice how the names include a verb in the imperative mood.

## Exceptions

An important part of the modeling process is thinking about the things that can cause a command to be refused. The Edument CQRS Starter Kit is decidedly opinionated here: we are expected to model these "sad paths" into exception types, just as commands and events are expressed as DTOs. Furthermore, these exception types may carry details of why the request was not acceptable. This is because the domain logic should tell the frontend what was wrong, rather than leaving it to ask by trying to inspect some state - or worse, leave the user to guess.

Looking to the scenario, we can identify three notable exceptions we need in our model:

    CannotCancelServedItem
    TabHasUnservedItems
    MustPayEnough

The names here try to explain why the command failed.

## Aggregates

Of course, man does not live by verbs alone. At some point, there must be nouns. More concretely, there has to be a way to talk about current state in order to decide if a command should be accepted. For example, to refuse to cancel a served item, we have to know that it was served.

All of the information we need is captured in the stream of past events, since they capture all facts introduced into the system. However, we are not, in general, intersted in the whole stream. Instead, we're interested in the events that relate, for example, to a particular tab. It's as if each tab should have its own event stream.

The missing piece here is known as an aggregate. Each aggregate has its own stream of events. Taken together, they can be used to compute its current state. Aggregates are completely isolated from each other. Decisions about whether to accept a command are made solely on the basis of the command itself and the information contained in the aggregate's past events.

Concretely, an aggregate is either:

    A single object, which doesn't reference any others.
    An isolated graph of objects, with one object designated as the root of the aggregate. The outside world should only know about the root.

This is demanding from a design perspective, as it forces us to identify and segregate business concepts. It also means we have to really focus on consistency boundaries.

Our current example has been carefully selected to have just one aggregate type - namely, a tab. However, in most systems you will have more work to do in order to identify aggregates. The authors of this tutorial have found that starting from the events and commands, then trying to group them based on invariants (business rules you need to uphold), is a good strategy.

## Running the tests

First, run the `init_db.sh` script in the [scripts](./scripts) directory to bring up a PostgreSQL docker container. Then run

```cargo test```