LDA #$FE
TAX
STA $0001
ADC #$01
ADC #$01
JMP end
end:
;<A> = 0x1A
;<X> = <A>
;ROM[0x0001] = <A>
;<A> = <A> + ROM[0x01]
;<PC> = end
;ROM[0x0002] = <A>