use array::ArrayTrait;
use array::SpanTrait;
use option::OptionTrait;
use serde::Serde;
use serde::deserialize_array_helper;
use serde::serialize_array_helper;
use starknet::ContractAddress;

const IERC165_ID: u32 = 0x01ffc9a7_u32;
const IERC721_ID: u32 = 0x80ac58cd_u32;
const IERC721_METADATA_ID: u32 = 0x5b5e139f_u32;
const IERC721_RECEIVER_ID: u32 = 0x150b7a02_u32;


#[abi]
trait IERC721 {
    fn balance_of(account: ContractAddress) -> u256;
    fn owner_of(token_id: u256) -> ContractAddress;
    fn transfer_from(from: ContractAddress, to: ContractAddress, token_id: u256);
    fn safe_transfer_from(
        from: ContractAddress, to: ContractAddress, token_id: u256, data: Span<felt252>
    );
    fn approve(to: ContractAddress, token_id: u256);
    fn set_approval_for_all(operator: ContractAddress, approved: bool);
    fn get_approved(token_id: u256) -> ContractAddress;
    fn is_approved_for_all(owner: ContractAddress, operator: ContractAddress) -> bool;
    // IERC721Metadata
    fn name() -> felt252;
    fn symbol() -> felt252;
    fn token_uri(token_id: u256) -> felt252;
}

#[abi]
trait IERC721Camel {
    fn balanceOf(account: ContractAddress) -> u256;
    fn ownerOf(tokenId: u256) -> ContractAddress;
    fn transferFrom(from: ContractAddress, to: ContractAddress, tokenId: u256);
    fn safeTransferFrom(
        from: ContractAddress, to: ContractAddress, tokenId: u256, data: Span<felt252>
    );
    fn approve(to: ContractAddress, tokenId: u256);
    fn setApprovalForAll(operator: ContractAddress, approved: bool);
    fn getApproved(tokenId: u256) -> ContractAddress;
    fn isApprovedForAll(owner: ContractAddress, operator: ContractAddress) -> bool;
    // IERC721Metadata
    fn name() -> felt252;
    fn symbol() -> felt252;
    fn tokenUri(tokenId: u256) -> felt252;
}

#[abi]
trait IASTRONFT  {
    fn mint(to: ContractAddress, token_id: u256);
}

//
// ERC721Receiver
//

#[abi]
trait IERC721ReceiverABI {
    fn on_erc721_received(
        operator: ContractAddress, from: ContractAddress, token_id: u256, data: Span<felt252>
    ) -> u32;
    fn onERC721Received(
        operator: ContractAddress, from: ContractAddress, tokenId: u256, data: Span<felt252>
    ) -> u32;
}

#[abi]
trait IERC721Receiver {
    fn on_erc721_received(
        operator: ContractAddress, from: ContractAddress, token_id: u256, data: Span<felt252>
    ) -> u32;
}

#[abi]
trait IERC721ReceiverCamel {
    fn onERC721Received(
        operator: ContractAddress, from: ContractAddress, tokenId: u256, data: Span<felt252>
    ) -> u32;
}

#[abi]
trait ERC721ABI {
    // case agnostic
    #[view]
    fn name() -> felt252;
    #[view]
    fn symbol() -> felt252;
    #[external]
    fn approve(to: ContractAddress, token_id: u256);
    // snake_case
    #[view]
    fn balance_of(account: ContractAddress) -> u256;
    #[view]
    fn owner_of(token_id: u256) -> ContractAddress;
    #[external]
    fn transfer_from(from: ContractAddress, to: ContractAddress, token_id: u256);
    #[external]
    fn safe_transfer_from(
        from: ContractAddress, to: ContractAddress, token_id: u256, data: Span<felt252>
    );
    #[external]
    fn set_approval_for_all(operator: ContractAddress, approved: bool);
    #[view]
    fn get_approved(token_id: u256) -> ContractAddress;
    #[view]
    fn is_approved_for_all(owner: ContractAddress, operator: ContractAddress) -> bool;
    #[view]
    fn token_uri(token_id: u256) -> felt252;
    // camelCase
    #[view]
    fn balanceOf(account: ContractAddress) -> u256;
    #[view]
    fn ownerOf(tokenId: u256) -> ContractAddress;
    #[external]
    fn transferFrom(from: ContractAddress, to: ContractAddress, tokenId: u256);
    #[external]
    fn safeTransferFrom(
        from: ContractAddress, to: ContractAddress, tokenId: u256, data: Span<felt252>
    );
    #[external]
    fn setApprovalForAll(operator: ContractAddress, approved: bool);
    #[view]
    fn getApproved(tokenId: u256) -> ContractAddress;
    #[view]
    fn isApprovedForAll(owner: ContractAddress, operator: ContractAddress) -> bool;
    #[view]
    fn tokenUri(tokenId: u256) -> felt252;
}

#[abi]
trait ASTRONFTABI {
    #[external]
    fn mint(to: ContractAddress, token_id: u256);
}


#[contract]
mod ERC721 {
    use super::IERC721;
    use super::IERC721Camel;
    use super::IASTRONFT;

    // Other
    use starknet::ContractAddress;
    use starknet::get_caller_address;
    use zeroable::Zeroable;
    use option::OptionTrait;
    use array::SpanTrait;
    use traits::Into;
    use super::SpanSerde;

    struct Storage {
        _name: felt252,
        _symbol: felt252,
        _owners: LegacyMap<u256, ContractAddress>,
        _balances: LegacyMap<ContractAddress, u256>,
        _token_approvals: LegacyMap<u256, ContractAddress>,
        _operator_approvals: LegacyMap<(ContractAddress, ContractAddress), bool>,
        _token_uri: LegacyMap<u256, felt252>,
    }

    #[event]
    fn Transfer(from: ContractAddress, to: ContractAddress, token_id: u256) {}

    #[event]
    fn Approval(owner: ContractAddress, approved: ContractAddress, token_id: u256) {}

    #[event]
    fn ApprovalForAll(owner: ContractAddress, operator: ContractAddress, approved: bool) {}

    #[constructor]
    fn constructor(name: felt252, symbol: felt252) {
        initializer(name, symbol);
    }

    impl IASTRONFTImpl of IASTRONFT {
        fn mint(to: ContractAddress, token_id: u256) {
            _mint(to, token_id);
        }
    }

    impl ERC721Impl of IERC721 {
        fn name() -> felt252 {
            _name::read()
        }

        fn symbol() -> felt252 {
            _symbol::read()
        }

        fn token_uri(token_id: u256) -> felt252 {
            assert(_exists(token_id), 'ERC721: invalid token ID');
            _token_uri::read(token_id)
        }

        fn balance_of(account: ContractAddress) -> u256 {
            assert(!account.is_zero(), 'ERC721: invalid account');
            _balances::read(account)
        }

        fn owner_of(token_id: u256) -> ContractAddress {
            _owner_of(token_id)
        }

        fn get_approved(token_id: u256) -> ContractAddress {
            assert(_exists(token_id), 'ERC721: invalid token ID');
            _token_approvals::read(token_id)
        }

        fn is_approved_for_all(owner: ContractAddress, operator: ContractAddress) -> bool {
            _operator_approvals::read((owner, operator))
        }

        fn approve(to: ContractAddress, token_id: u256) {
            let owner = _owner_of(token_id);

            let caller = get_caller_address();
            assert(
                owner == caller | is_approved_for_all(owner, caller), 'ERC721: unauthorized caller'
            );
            _approve(to, token_id);
        }

        fn set_approval_for_all(operator: ContractAddress, approved: bool) {
            _set_approval_for_all(get_caller_address(), operator, approved)
        }

        fn transfer_from(from: ContractAddress, to: ContractAddress, token_id: u256) {
            assert(
                _is_approved_or_owner(get_caller_address(), token_id), 'ERC721: unauthorized caller'
            );
            _transfer(from, to, token_id);
        }

        fn safe_transfer_from(
            from: ContractAddress, to: ContractAddress, token_id: u256, data: Span<felt252>
        ) {
            assert(
                _is_approved_or_owner(get_caller_address(), token_id), 'ERC721: unauthorized caller'
            );
            _safe_transfer(from, to, token_id, data);
        }
    }

    impl ERC721CamelImpl of IERC721Camel {
        fn name() -> felt252 {
            ERC721Impl::name()
        }

        fn symbol() -> felt252 {
            ERC721Impl::symbol()
        }

        fn tokenUri(tokenId: u256) -> felt252 {
            ERC721Impl::token_uri(tokenId)
        }

        fn balanceOf(account: ContractAddress) -> u256 {
            ERC721Impl::balance_of(account)
        }

        fn ownerOf(tokenId: u256) -> ContractAddress {
            ERC721Impl::owner_of(tokenId)
        }

        fn approve(to: ContractAddress, tokenId: u256) {
            ERC721Impl::approve(to, tokenId)
        }

        fn getApproved(tokenId: u256) -> ContractAddress {
            ERC721Impl::get_approved(tokenId)
        }

        fn isApprovedForAll(owner: ContractAddress, operator: ContractAddress) -> bool {
            ERC721Impl::is_approved_for_all(owner, operator)
        }

        fn setApprovalForAll(operator: ContractAddress, approved: bool) {
            ERC721Impl::set_approval_for_all(operator, approved)
        }

        fn transferFrom(from: ContractAddress, to: ContractAddress, tokenId: u256) {
            ERC721Impl::transfer_from(from, to, tokenId)
        }

        fn safeTransferFrom(
            from: ContractAddress, to: ContractAddress, tokenId: u256, data: Span<felt252>
        ) {
            ERC721Impl::safe_transfer_from(from, to, tokenId, data)
        }
    }

    // View

    #[view]
    fn supports_interface(interface_id: u32) -> bool {
        if super::IERC165_ID == interface_id {
            return true;
        } else if super::IERC721_METADATA_ID == interface_id {
            return true;
        } else {
            return super::IERC721_ID == interface_id;
        }
    }

    #[view]
    fn supportsInterface(interfaceId: u32) -> bool {
        if super::IERC165_ID == interfaceId {
            return true;
        } else if super::IERC721_METADATA_ID == interfaceId {
            return true;
        } else {
            return super::IERC721_ID == interfaceId;
        }
    }

    #[view]
    fn name() -> felt252 {
        ERC721Impl::name()
    }

    #[view]
    fn symbol() -> felt252 {
        ERC721Impl::symbol()
    }

    #[view]
    fn token_uri(token_id: u256) -> felt252 {
        ERC721Impl::token_uri(token_id)
    }

    #[view]
    fn tokenUri(tokenId: u256) -> felt252 {
        ERC721CamelImpl::tokenUri(tokenId)
    }

    #[view]
    fn balance_of(account: ContractAddress) -> u256 {
        ERC721Impl::balance_of(account)
    }

    #[view]
    fn balanceOf(account: ContractAddress) -> u256 {
        ERC721CamelImpl::balanceOf(account)
    }

    #[view]
    fn owner_of(token_id: u256) -> ContractAddress {
        ERC721Impl::owner_of(token_id)
    }

    #[view]
    fn ownerOf(tokenId: u256) -> ContractAddress {
        ERC721CamelImpl::ownerOf(tokenId)
    }

    #[view]
    fn get_approved(token_id: u256) -> ContractAddress {
        ERC721Impl::get_approved(token_id)
    }

    #[view]
    fn getApproved(tokenId: u256) -> ContractAddress {
        ERC721CamelImpl::getApproved(tokenId)
    }

    #[view]
    fn is_approved_for_all(owner: ContractAddress, operator: ContractAddress) -> bool {
        ERC721Impl::is_approved_for_all(owner, operator)
    }

    #[view]
    fn isApprovedForAll(owner: ContractAddress, operator: ContractAddress) -> bool {
        ERC721CamelImpl::isApprovedForAll(owner, operator)
    }

    // External

    #[external]
    fn approve(to: ContractAddress, token_id: u256) {
        ERC721Impl::approve(to, token_id)
    }

    #[external]
    fn set_approval_for_all(operator: ContractAddress, approved: bool) {
        ERC721Impl::set_approval_for_all(operator, approved)
    }

    #[external]
    fn setApprovalForAll(operator: ContractAddress, approved: bool) {
        ERC721CamelImpl::setApprovalForAll(operator, approved)
    }

    #[external]
    fn transfer_from(from: ContractAddress, to: ContractAddress, token_id: u256) {
        ERC721Impl::transfer_from(from, to, token_id)
    }

    #[external]
    fn transferFrom(from: ContractAddress, to: ContractAddress, tokenId: u256) {
        ERC721CamelImpl::transferFrom(from, to, tokenId)
    }

    #[external]
    fn safe_transfer_from(
        from: ContractAddress, to: ContractAddress, token_id: u256, data: Span<felt252>
    ) {
        ERC721Impl::safe_transfer_from(from, to, token_id, data)
    }

    #[external]
    fn safeTransferFrom(
        from: ContractAddress, to: ContractAddress, tokenId: u256, data: Span<felt252>
    ) {
        ERC721CamelImpl::safeTransferFrom(from, to, tokenId, data)
    }

    #[external]
    fn mint(to: ContractAddress, token_id: u256) {
            IASTRONFTImpl::mint(to, token_id);
    }
    
    // Internal

    #[internal]
    fn initializer(name_: felt252, symbol_: felt252) {
        _name::write(name_);
        _symbol::write(symbol_);
    }

    #[internal]
    fn _owner_of(token_id: u256) -> ContractAddress {
        let owner = _owners::read(token_id);
        match owner.is_zero() {
            bool::False(()) => owner,
            bool::True(()) => panic_with_felt252('ERC721: invalid token ID')
        }
    }

    #[internal]
    fn _exists(token_id: u256) -> bool {
        !_owners::read(token_id).is_zero()
    }

    #[internal]
    fn _is_approved_or_owner(spender: ContractAddress, token_id: u256) -> bool {
        let owner = _owner_of(token_id);
        owner == spender | is_approved_for_all(owner, spender) | spender == get_approved(token_id)
    }

    #[internal]
    fn _approve(to: ContractAddress, token_id: u256) {
        let owner = _owner_of(token_id);
        assert(owner != to, 'ERC721: approval to owner');
        _token_approvals::write(token_id, to);
        Approval(owner, to, token_id);
    }

    #[internal]
    fn _set_approval_for_all(owner: ContractAddress, operator: ContractAddress, approved: bool) {
        assert(owner != operator, 'ERC721: self approval');
        _operator_approvals::write((owner, operator), approved);
        ApprovalForAll(owner, operator, approved);
    }

    #[internal]
    fn _mint(to: ContractAddress, token_id: u256) {
        assert(!to.is_zero(), 'ERC721: invalid receiver');
        assert(!_exists(token_id), 'ERC721: token already minted');

        // Update balances
        _balances::write(to, _balances::read(to) + 1.into());

        // Update token_id owner
        _owners::write(token_id, to);

        // Emit event
        Transfer(Zeroable::zero(), to, token_id);
    }

    #[internal]
    fn _transfer(from: ContractAddress, to: ContractAddress, token_id: u256) {
        assert(!to.is_zero(), 'ERC721: invalid receiver');
        let owner = _owner_of(token_id);
        assert(from == owner, 'ERC721: wrong sender');

        // Implicit clear approvals, no need to emit an event
        _token_approvals::write(token_id, Zeroable::zero());

        // Update balances
        _balances::write(from, _balances::read(from) - 1.into());
        _balances::write(to, _balances::read(to) + 1.into());

        // Update token_id owner
        _owners::write(token_id, to);

        // Emit event
        Transfer(from, to, token_id);
    }

    #[internal]
    fn _burn(token_id: u256) {
        let owner = _owner_of(token_id);

        // Implicit clear approvals, no need to emit an event
        _token_approvals::write(token_id, Zeroable::zero());

        // Update balances
        _balances::write(owner, _balances::read(owner) - 1.into());

        // Delete owner
        _owners::write(token_id, Zeroable::zero());

        // Emit event
        Transfer(owner, Zeroable::zero(), token_id);
    }

    #[internal]
    fn _safe_mint(to: ContractAddress, token_id: u256, data: Span<felt252>) {
        _mint(to, token_id);
    }

    #[internal]
    fn _safe_transfer(
        from: ContractAddress, to: ContractAddress, token_id: u256, data: Span<felt252>
    ) {
        _transfer(from, to, token_id);
    }

    #[internal]
    fn _set_token_uri(token_id: u256, token_uri: felt252) {
        assert(_exists(token_id), 'ERC721: invalid token ID');
        _token_uri::write(token_id, token_uri)
    }
}

impl SpanSerde<
    T, impl TSerde: Serde<T>, impl TCopy: Copy<T>, impl TDrop: Drop<T>
> of Serde<Span<T>> {
    fn serialize(self: @Span<T>, ref output: Array<felt252>) {
        (*self).len().serialize(ref output);
        serialize_array_helper(*self, ref output);
    }
    fn deserialize(ref serialized: Span<felt252>) -> Option<Span<T>> {
        let length = *serialized.pop_front()?;
        let mut arr = ArrayTrait::new();
        Option::Some(deserialize_array_helper(ref serialized, arr, length)?.span())
    }
}
