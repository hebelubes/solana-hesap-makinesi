# Solana Hesap Makinesi

Bu proje, Solana blockchain’inde çalışan bir hesap makinesi programıdır. Toplama, çıkarma, çarpma ve bölme işlemlerini yapar ve sonucu bir hesapta saklar.

## Özellikler
- Temel matematiksel işlemler.
- Sonuçları Solana account’unda saklama.
- Hata kontrolü (negatif sonuç ve sıfıra bölme).

## Nasıl Kullanılır
1. [Anchor Playground](https://beta.anchor-lang.com/)’da projeyi açın.
2. `programs/calculator/src/lib.rs` ve `Anchor.toml` dosyalarını yükleyin.
3. `anchor build` ve `anchor deploy` ile programı derleyin ve deploy edin.
4. `tests/calculator.ts` ile test edin.

## Teknolojiler
- Rust, Anchor Framework
- Solana Blockchain
- TypeScript (test için)

## Lisans
MIT License
