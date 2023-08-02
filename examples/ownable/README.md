## Ownable contract

Contract module which provides a basic access control mechanism, where there is an account (an owner) that can be granted exclusive access to specific functions.

This also aim to show how to use some functions of Openbrush with some of Allfeat_contract together.

This module is used through the embedding of ownable::Data and implementation of Ownable and Storage openbrush traits. It will make the modifier only_owner available, which can be applied to your functions to restrict their use to the owner.