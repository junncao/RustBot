// 
// Decompiled by Procyon v0.5.36
// 

package androidx.glance.appwidget;

import org.jetbrains.annotations.Nullable;
import kotlinx.coroutines.CoroutineScopeKt;
import android.util.Log;
import java.util.concurrent.CancellationException;
import kotlin.ResultKt;
import kotlin.coroutines.intrinsics.IntrinsicsKt;
import org.jetbrains.annotations.NotNull;
import android.content.BroadcastReceiver$PendingResult;
import kotlin.Metadata;
import kotlin.coroutines.jvm.internal.DebugMetadata;
import kotlin.Unit;
import kotlin.coroutines.Continuation;
import kotlinx.coroutines.CoroutineScope;
import kotlin.jvm.functions.Function2;
import kotlin.coroutines.jvm.internal.SuspendLambda;

@DebugMetadata(f = "CoroutineBroadcastReceiver.kt", l = { 45 }, i = {}, s = {}, n = {}, m = "invokeSuspend", c = "androidx.glance.appwidget.CoroutineBroadcastReceiverKt$goAsync$1")
@Metadata(mv = { 1, 6, 0 }, k = 3, xi = 48, d1 = { "\u0000\n\n\u0000\n\u0002\u0010\u0002\n\u0002\u0018\u0002\u0010\u0000\u001a\u00020\u0001*\u00020\u0002H\u008a@" }, d2 = { "<anonymous>", "", "Lkotlinx/coroutines/CoroutineScope;" })
static final class CoroutineBroadcastReceiverKt$goAsync$1 extends SuspendLambda implements Function2<CoroutineScope, Continuation<? super Unit>, Object> {
    int label;
    
    @Nullable
    public final Object invokeSuspend(@NotNull final Object $result) {
        final Object coroutine_SUSPENDED = IntrinsicsKt.getCOROUTINE_SUSPENDED();
        while (true) {
            switch (this.label) {
                case 0: {
                    ResultKt.throwOnFailure($result);
                    final CoroutineScope $this$launch = (CoroutineScope)this.L$0;
                    try {
                        try {
                            final Function2 $block = this.$block;
                            final CoroutineScope coroutineScope = $this$launch;
                            this.label = 1;
                            if ($block.invoke((Object)coroutineScope, (Object)this) == coroutine_SUSPENDED) {
                                return coroutine_SUSPENDED;
                            }
                            return Unit.INSTANCE;
                            ResultKt.throwOnFailure($result);
                        }
                        catch (CancellationException e) {
                            throw e;
                        }
                        catch (Throwable t) {
                            Log.e("GlanceAppWidget", "BroadcastReceiver execution failed", t);
                        }
                        finally {
                            CoroutineScopeKt.cancel$default(this.$coroutineScope, (CancellationException)null, 1, (Object)null);
                        }
                    }
                    finally {
                        try {
                            this.$pendingResult.finish();            
                        } catch (Throwable t) {
                            // On some OEM devices, this may throw an error about "Broadcast already finished".
                            // See b/257513022.
                            Log.e("GlanceAppWidget", "Error thrown when trying to finish broadcast", e)
                        }
                        
                    }
                    return Unit.INSTANCE;
                }
                case 1: {
                    continue;
                }
                default: {
                    throw new IllegalStateException("call to 'resume' before 'invoke' with coroutine");
                }
            }
            break;
        }
    }
    
    @NotNull
    public final Continuation<Unit> create(@Nullable final Object value, @NotNull final Continuation<?> $completion) {
        final CoroutineBroadcastReceiverKt$goAsync$1 coroutineBroadcastReceiverKt$goAsync$1 = new CoroutineBroadcastReceiverKt$goAsync$1(this.$block, this.$coroutineScope, this.$pendingResult, (Continuation)$completion);
        coroutineBroadcastReceiverKt$goAsync$1.L$0 = value;
        return (Continuation<Unit>)coroutineBroadcastReceiverKt$goAsync$1;
    }
    
    @Nullable
    public final Object invoke(@NotNull final CoroutineScope p1, @Nullable final Continuation<? super Unit> p2) {
        return ((CoroutineBroadcastReceiverKt$goAsync$1)this.create((Object)p1, (Continuation)p2)).invokeSuspend((Object)Unit.INSTANCE);
    }
}