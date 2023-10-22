use scrypto::prelude::*;

#[derive(NonFungibleData, ScryptoSbor)]
pub struct Eco {
    #[mutable]
    name: String,
}

#[blueprint]
mod ecobadge {
    enable_method_auth! {
        roles {
            owner => updatable_by: [];
        },

        methods {
            burn_eco_badge => restrict_to: [owner];
            get_eco_badge => PUBLIC; 
        }
    }

    struct Ecobadge {
        // The resource address for the owner badge
        eco_owner_badge: ResourceAddress,
        // The resource manager address for the NFTs (badges)
        eco_badge_resource_manager: ResourceManager,
        // A counter for ID generation
        eco_badge_id_counter: u64,
    }

    impl Ecobadge {
        // creates the component and the resource manager    
        pub fn instantiate_ecobadge() -> FungibleBucket  {   
            let (address_reservation, component_address) =
                Runtime::allocate_component_address(<Ecobadge>::blueprint_id());

            // create an owner badge        
            let eco_owner_badge = ResourceBuilder::new_fungible(OwnerRole::None)
            .metadata(metadata!(
                init {
                    "name" => "EcoBadge Owner".to_string(), locked;
                }
            ))
            .divisibility(DIVISIBILITY_NONE)
            .mint_initial_supply(1);

            let eco_badge_resource_manager = ResourceBuilder::new_integer_non_fungible::<Eco>(OwnerRole::None)
            .metadata(metadata! {
                init {
                    "name" => "Ecosystem Badge", locked;
                    "description" => "Individual badge to access the project section of RadixCharts", locked;
                }
            })
            .burn_roles(burn_roles! {
                burner => rule!(require(global_caller(component_address)));
                burner_updater => rule!(deny_all);
            })
            .mint_roles(mint_roles!{
                minter => rule!(allow_all);
                minter_updater => rule!(deny_all);
            })
            .recall_roles(recall_roles!{
                recaller => rule!(require(eco_owner_badge.resource_address()));
                recaller_updater => rule!(deny_all);
            })
            .create_with_no_initial_supply();

            // Instantiate an Ecobadge component
            Self {
                eco_owner_badge: eco_owner_badge.resource_address(),
                eco_badge_resource_manager,
                eco_badge_id_counter: 1,
            }
            .instantiate()    
            .prepare_to_globalize(
                OwnerRole::Fixed(
                    rule!(require(eco_owner_badge.resource_address())
                )
            ))
            .roles(roles!(
                    owner => rule!(require(eco_owner_badge.resource_address()));
            ))        
            .enable_component_royalties(component_royalties! {
                init {
                    get_eco_badge => Usd(5.into()), updatable;
                    burn_eco_badge => Free, updatable;
                }
            })
            .with_address(address_reservation)
            .globalize();

            // return the owner badge   
            eco_owner_badge

        }

        // Method: Mint a new non fungible token that can be used to represent a badge of ownership of a project in the ecosystem
        // Receives: project name 
        // Returns: the newly minted badge
         pub fn get_eco_badge(&mut self, project: String) -> Bucket {
            let data = Eco {
                name: project,
            };
            // mint a new non fungible token          
            let nft_bucket = self.eco_badge_resource_manager.mint_non_fungible(
                &NonFungibleLocalId::Integer(self.eco_badge_id_counter.into()),
                data,
            );
            self.eco_badge_id_counter += 1;
            
            // return the newly minted badge
            nft_bucket
        }

        // Method: Burns a ecosystem badge if no longer needed
        // Receives: ID of the badge to burn 
        // Returns: none
         pub fn burn_eco_badge(&mut self, nft_bucket: Bucket) {
            // Burn the badge
            nft_bucket.burn();
        }
    }
}
