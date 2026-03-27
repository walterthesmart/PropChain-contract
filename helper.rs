        fn check_signature_threshold(
            &self,
            escrow_id: u64,
            approval_type: ApprovalType,
        ) -> Result<bool, Error> {
            let config = self
                .multi_sig_configs
                .get(&escrow_id)
                .ok_or(Error::EscrowNotFound)?;
            let count = self
                .signature_counts
                .get(&(escrow_id, approval_type))
                .unwrap_or(0);
            Ok(count >= config.required_signatures)
        }