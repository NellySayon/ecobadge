# Ecobadge bluepint

Simple blueprint that mints a non fungible token which can be used as access token to your website.
Using access roles and royalties.

### Created resources
1. A fungible resource used as owner badge
2. A Resource Manager used for minting the NFTs

### Access control
1. Burning is just allowed from inside the component
2. Recall is only allowed by the owner (with the owner_badge)


## How to install

1. Install the latest Scrypto version and prepare your IDE as described in the official documentation.

2. Clone this repository

3. Run ```Scrypto build```

4. Go to the developer console and publish the package.


## Transaction Mainfest examples

### Within your website

  ```
  // prepare the transaction manifest to mint a new badge
  let manifest = `
  CALL_METHOD
    Address("${process.env.COMPONENT_ADDRESS}")
    "get_eco_badge"
    "${projectName}"
  ;
  CALL_METHOD
    Address("${accountAddress}")
    "deposit_batch"
    Expression("ENTIRE_WORKTOP");
    `;
  console.log("Manifest: ", manifest);

  // send the manifest to the wallet (via the extension) for signing
  const result = await rdt.walletApi.sendTransaction({
    transactionManifest: manifest,
    version: 1,
  });
  if (result.isErr()) throw result.error;
  console.log("WalletSDK Result: ", result.value);
  ```

### Via the console

#### Claim Royalties
```
CALL_METHOD
    Address("<owner_account>")
    "create_proof_of_amount"
    Address("<owner_badge_resource>")
    Decimal("1")
;

CLAIM_COMPONENT_ROYALTIES
    Address("<component>");

CALL_METHOD
    Address("<receiving_account>")
    "deposit_batch"
    Expression("ENTIRE_WORKTOP")
;
```


#### Recall a badge
```
CALL_METHOD
    Address("<owner_account>")
    "create_proof_of_amount"
    Address("<owner_badge_resource>")
    Decimal("1")
;

RECALL_NON_FUNGIBLES_FROM_VAULT
    Address("<internal_vault>")
    Array<NonFungibleLocalId>(
        NonFungibleLocalId("<number>")
    )
;

CALL_METHOD
    Address("<receiving_account>")
    "deposit_batch"
    Expression("ENTIRE_WORKTOP")
;
```

#### Burn a badge
```
CALL_METHOD
    Address("<account>")
    "withdraw_non_fungibles"
    Address("<resource>")
    Array<NonFungibleLocalId>(
        NonFungibleLocalId("<number>")
    )
;

TAKE_NON_FUNGIBLES_FROM_WORKTOP
    Address("<resource>")
    Array<NonFungibleLocalId>(
        NonFungibleLocalId("<number>")
    )
    Bucket("bucket1")
;

CALL_METHOD
    Address("<component>")
    "burn_eco_badge"
     Bucket("bucket1")
;
```
